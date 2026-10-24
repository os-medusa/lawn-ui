import fs from 'fs';

const doc = JSON.parse(fs.readFileSync('C:/Users/fakelog/.gemini/antigravity/brain/11c44f6c-7614-443d-82ad-6d29dac273c7/figcraft_extracted/document.json', 'utf8'));

const vars = {};
doc.variables.variables.forEach(v => vars[v.id] = v);

function getVar(id) { return vars[id]; }
function getVarName(id) { return vars[id]?.name || null; }

function toHex(c) {
  if (!c) return 'none';
  const r = Math.round(c.r * 255).toString(16).padStart(2, '0');
  const g = Math.round(c.g * 255).toString(16).padStart(2, '0');
  const b = Math.round(c.b * 255).toString(16).padStart(2, '0');
  return '#' + r + g + b;
}

function resolveColor(varId, mode = 'Light') {
  const v = vars[varId];
  if (!v) return null;
  const modeKey = mode === 'Light' ? 'VariableCollectionId:1/mode1' : 'VariableCollectionId:1/mode2';
  const modeKeyUX = mode === 'Light' ? 'VariableCollectionId:4/mode1' : 'VariableCollectionId:4/mode2';
  const val = v.values[modeKey] || v.values[modeKeyUX];
  if (!val || !val.value) return null;
  return {
    hex: toHex(val.value),
    opacity: val.value.a !== undefined ? val.value.a : 1,
    token: v.name
  };
}

function resolveNumber(varId) {
  const v = vars[varId];
  if (!v) return null;
  const val = v.values['VariableCollectionId:2/mode1'] || v.values['VariableCollectionId:3/mode1'];
  return val ? val.value : null;
}

function resolveString(varId) {
  const v = vars[varId];
  if (!v) return null;
  const val = v.values['VariableCollectionId:3/mode1'];
  return val ? val.value : null;
}

const compIds = [
  { id: 1044, name: 'Button' },
  { id: 1087, name: 'Input' },
  { id: 1126, name: 'Select' },
  { id: 1145, name: 'Textarea' },
  { id: 1180, name: 'Badge' },
  { id: 1215, name: 'Switch' },
  { id: 1248, name: 'Checkbox' },
  { id: 1271, name: 'Radio group' },
  { id: 1294, name: 'Slider' },
  { id: 1325, name: 'Tabs' },
  { id: 1328, name: 'Breadcrumb' },
  { id: 1348, name: 'Pagination' },
  { id: 1374, name: 'Dropdown menu' },
  { id: 1396, name: 'Accordion' },
  { id: 1426, name: 'Tooltip' },
  { id: 1436, name: 'Popover' },
  { id: 1480, name: 'Alert' },
  { id: 1507, name: 'Progress' },
  { id: 1534, name: 'Avatar' },
  { id: 1537, name: 'Skeleton' },
  { id: 1549, name: 'Card' },
  { id: 1565, name: 'Dialog' },
  { id: 1589, name: 'Table' },
  { id: 1635, name: 'Sidebar' },
  { id: 1689, name: 'Calendar' },
  { id: 1823, name: 'Item' },
  { id: 1826, name: 'Empty state' }
];

const results = [];

for (const item of compIds) {
  const node = doc.nodes[item.id];
  if (!node) continue;
  
  const compData = {
    id: item.id,
    name: item.name,
    rawName: node.name,
    type: node.type,
    variants: []
  };

  const childrenNodes = (node.type === 'componentSet') 
    ? node.children.map(cid => doc.nodes[cid])
    : [node];

  for (const v of childrenNodes) {
    if (!v) continue;

    // Search for visual elements within variant
    // either v itself or its primary child (Control / Surface)
    let visualNode = v;
    let textNode = null;
    let iconNode = null;

    function findNodes(n, depth = 0) {
      if (depth > 0 && n.type === 'text' && !textNode) textNode = n;
      if (depth > 0 && (n.name.includes('Lucide') || n.name.includes('Icon')) && !iconNode) iconNode = n;
      if (n.corners || n.fills?.some(f => f.opacity > 0) || n.strokes?.length > 0) {
        if (visualNode === v && n !== v) {
          visualNode = n;
        }
      }
      if (n.children) {
        n.children.forEach(cid => {
          const ch = doc.nodes[cid];
          if (ch) findNodes(ch, depth + 1);
        });
      }
    }
    findNodes(v);

    // Radii
    const radii = visualNode.corners?.radii || v.corners?.radii || null;
    const rToken = getVarName(visualNode.bound?.cornerRadius || v.bound?.cornerRadius);

    // Padding & Gap
    const layoutNode = (v.layout ? v : (visualNode.layout ? visualNode : null));
    const padding = layoutNode?.layout?.padding || null;
    const gap = layoutNode?.layout?.gap ?? null;

    // Exact Size
    const width = v.width;
    const height = visualNode.height || v.height;
    const hToken = getVarName(visualNode.bound?.height || v.bound?.height);

    // Fills & Strokes
    const fillToken = getVarName(visualNode.bound?.fill || v.bound?.fill);
    const strokeToken = getVarName(visualNode.bound?.stroke || v.bound?.stroke);
    
    // Light & Dark color resolution
    const fillVarId = visualNode.bound?.fill || v.bound?.fill;
    const strokeVarId = visualNode.bound?.stroke || v.bound?.stroke;

    const fillLight = fillVarId ? resolveColor(fillVarId, 'Light') : (visualNode.fills?.[0] ? { hex: toHex(visualNode.fills[0].color), opacity: visualNode.fills[0].opacity ?? 1, token: null } : null);
    const fillDark = fillVarId ? resolveColor(fillVarId, 'Dark') : null;

    const strokeLight = strokeVarId ? resolveColor(strokeVarId, 'Light') : (visualNode.strokes?.[0] ? { hex: toHex(visualNode.strokes[0].color), opacity: visualNode.strokes[0].opacity ?? 1, token: null } : null);
    const strokeDark = strokeVarId ? resolveColor(strokeVarId, 'Dark') : null;

    // Typography
    let typography = null;
    if (textNode) {
      typography = {
        fontFamily: textNode.style?.font_family || resolveString(textNode.bound?.fontFamily),
        fontSize: textNode.style?.font_size || resolveNumber(textNode.bound?.fontSize),
        fontSizeToken: getVarName(textNode.bound?.fontSize),
        lineHeight: textNode.style?.line_height?.value || null,
        fontWeight: textNode.style?.font_style || resolveString(textNode.bound?.fontWeight),
        textColorToken: getVarName(textNode.bound?.fill)
      };
    }

    compData.variants.push({
      variantId: v.id,
      variantName: v.name,
      visualNodeName: visualNode.name,
      width,
      height,
      hToken,
      radii,
      rToken,
      padding,
      gap,
      fillToken,
      fillLight,
      fillDark,
      strokeToken,
      strokeLight,
      strokeDark,
      typography
    });
  }

  results.push(compData);
}

fs.writeFileSync('C:/Users/fakelog/.gemini/antigravity/brain/11c44f6c-7614-443d-82ad-6d29dac273c7/ast_extracted_summary.json', JSON.stringify(results, null, 2));
console.log('Saved summary for ' + results.length + ' components.');
