/**
 * TI-83 Plus .8xp file generator
 * Creates valid calculator program files from assembled bytecode
 */

export class TI8XPGenerator {
  static create8xp(programName, code) {
    // TI-83 Plus file header
    const header = new Uint8Array([
      0x2a,
      0x2a,
      0x54,
      0x49,
      0x38,
      0x33,
      0x46,
      0x2a, // **TI83F*
      0x1a,
      0x0a,
      0x00, // signature
    ]);

    // Comment (42 bytes)
    const comment = new Uint8Array(42);
    const commentText = "Created by Z80 Assembler";
    for (let i = 0; i < Math.min(commentText.length, 42); i++) {
      comment[i] = commentText.charCodeAt(i);
    }

    // Build variable header
    const dataLength = code.length + 2;
    const varHeader = new Uint8Array(17);

    // Data section length (little-endian)
    varHeader[0] = (dataLength + 13) & 0xff;
    varHeader[1] = ((dataLength + 13) >> 8) & 0xff;

    // Variable header
    varHeader[2] = 0x0d;
    varHeader[3] = 0x00;
    varHeader[4] = dataLength & 0xff;
    varHeader[5] = (dataLength >> 8) & 0xff;
    varHeader[6] = 0x05; // Program type

    // Program name (8 bytes, padded)
    const name = programName.toUpperCase().substring(0, 8).padEnd(8, "\0");
    for (let i = 0; i < 8; i++) {
      varHeader[7 + i] = name.charCodeAt(i);
    }

    // Version and flag
    varHeader[15] = 0x00;
    varHeader[16] = 0x00;

    // Additional length bytes
    const lengthBytes = new Uint8Array([
      dataLength & 0xff,
      (dataLength >> 8) & 0xff,
    ]);

    // Program data (size + code)
    const programData = new Uint8Array(code.length + 2);
    programData[0] = code.length & 0xff;
    programData[1] = (code.length >> 8) & 0xff;
    programData.set(code, 2);

    // Calculate checksum
    let checksum = 0;
    for (let i = 2; i < varHeader.length; i++) {
      checksum += varHeader[i];
    }
    checksum += lengthBytes[0] + lengthBytes[1];
    for (let i = 0; i < programData.length; i++) {
      checksum += programData[i];
    }
    checksum &= 0xffff;

    // Build complete file
    const totalSize =
      header.length +
      comment.length +
      varHeader.length +
      lengthBytes.length +
      programData.length +
      2;
    const result = new Uint8Array(totalSize);
    let pos = 0;

    result.set(header, pos);
    pos += header.length;
    result.set(comment, pos);
    pos += comment.length;
    result.set(varHeader, pos);
    pos += varHeader.length;
    result.set(lengthBytes, pos);
    pos += lengthBytes.length;
    result.set(programData, pos);
    pos += programData.length;
    result[pos] = checksum & 0xff;
    result[pos + 1] = (checksum >> 8) & 0xff;

    return result;
  }
}

export default TI8XPGenerator;