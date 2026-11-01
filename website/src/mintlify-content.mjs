// Keep the imported MDX unchanged. Resolve its Mintlify components and root-relative
// documentation links during compilation, without touching code samples or heading IDs.
import { docsRoutes } from './navigation.mjs';

const legacyRoutes = new Set(docsRoutes.map((route) => route.replace(/^\/docs/, '').replace(/\/$/, '') || '/'));
const components = new Set(['Card', 'CardGroup', 'Frame', 'Note', 'Warning', 'Info', 'Tip']);

function docsLink(url) {
  const [, path, suffix] = url.match(/^([^?#]*)(.*)$/s);
  return legacyRoutes.has(path.replace(/\/$/, '') || '/') ? `/docs${path === '/' ? '/' : path}${suffix}` : url;
}

export default function mintlifyContent() {
  return (tree) => {
    const imports = new Map();
    function visit(node) {
      // Wide reference tables remain keyboard-scrollable on narrow layouts.
      if (node.type === 'table') node.data = { ...node.data, hProperties: { ...node.data?.hProperties, tabIndex: 0 } };
      if (['link', 'definition'].includes(node.type) && node.url?.startsWith('/') && !node.url.startsWith('//')) {
        node.url = docsLink(node.url);
      }
      if (node.type === 'code' && node.meta && !node.meta.includes('=')) node.meta = `title=${JSON.stringify(node.meta)}`;
      if (components.has(node.name)) {
        imports.set(node.name, `./components/mintlify/${node.name}.astro`);
        for (const attribute of node.attributes || []) {
          if (attribute.name === 'href' && typeof attribute.value === 'string' && attribute.value.startsWith('/')) {
            attribute.value = docsLink(attribute.value);
          }
        }
      }
      if (node.name === 'CodeGroup') {
        imports.set('Tabs', '@astrojs/starlight/components');
        imports.set('TabItem', '@astrojs/starlight/components');
        node.name = 'Tabs';
        node.children = node.children.filter((child) => child.type === 'code').map((child) => ({
          type: 'mdxJsxFlowElement', name: 'TabItem',
          attributes: [{ type: 'mdxJsxAttribute', name: 'label', value: child.meta || child.lang || 'Code' }],
          children: [child],
        }));
      }
      node.children?.forEach(visit);
    }
    visit(tree);
    if (!imports.size) return;
    tree.children.unshift({
      type: 'mdxjsEsm', value: '', data: { estree: {
        type: 'Program', sourceType: 'module', body: [...imports].map(([name, module]) => ({
          type: 'ImportDeclaration',
          source: { type: 'Literal', value: module.startsWith('.') ? new URL(module, import.meta.url).pathname : module },
          specifiers: [{
            type: module.startsWith('.') ? 'ImportDefaultSpecifier' : 'ImportSpecifier',
            local: { type: 'Identifier', name },
            ...(module.startsWith('.') ? {} : { imported: { type: 'Identifier', name } }),
          }],
        })),
      } },
    });
  };
}
