/**
 * Assembly line parser
 * Extracts labels, mnemonics, and operands from assembly source lines
 */

export class Parser {
  parseLine(line) {
    // Remove comments
    const commentIndex = line.indexOf(";");
    if (commentIndex !== -1) {
      line = line.substring(0, commentIndex);
    }
    line = line.trim();

    if (!line) return null;

    let label = "";
    let mnemonic = "";
    let operands = "";

    // Check for label
    const colonIndex = line.indexOf(":");
    if (colonIndex !== -1) {
      label = line.substring(0, colonIndex).trim();
      line = line.substring(colonIndex + 1).trim();
    }

    // Parse mnemonic and operands
    if (line) {
      // Special handling for bcall
      if (line.toLowerCase().startsWith("bcall(")) {
        mnemonic = "bcall";
        // Extract everything between bcall( and )
        const startIdx = line.indexOf("(");
        const endIdx = line.lastIndexOf(")");
        if (startIdx !== -1 && endIdx !== -1) {
          operands = line.substring(startIdx + 1, endIdx).trim();
        }
      } else {
        const spaceIndex = line.search(/\s/);
        if (spaceIndex === -1) {
          mnemonic = line.toLowerCase();
        } else {
          mnemonic = line.substring(0, spaceIndex).toLowerCase();
          operands = line.substring(spaceIndex + 1).trim();
        }
      }
    }

    return { label, mnemonic, operands };
  }

  /**
   * Split operands carefully to handle parentheses
   * e.g., "hl,($8000)" -> ["hl", "($8000)"]
   */
  splitOperands(operands) {
    const parts = [];
    let current = "";
    let parenDepth = 0;
    
    for (let i = 0; i < operands.length; i++) {
      const char = operands[i];
      if (char === "(") parenDepth++;
      if (char === ")") parenDepth--;
      if (char === "," && parenDepth === 0) {
        parts.push(current.trim());
        current = "";
      } else {
        current += char;
      }
    }
    parts.push(current.trim());
    
    return parts;
  }
}

export default Parser;