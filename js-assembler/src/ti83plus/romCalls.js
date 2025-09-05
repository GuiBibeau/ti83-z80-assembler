/**
 * TI-83 Plus ROM call addresses
 * Used with bcall instruction (RST 28h)
 */

export const romCalls = {
  _ClrLCDFull: 0x4540,
  _PutS: 0x450a,
  _PutC: 0x4504,
  _GetKey: 0x4972,
  _HomeUp: 0x4558,
  _NewLine: 0x452e,
  _ClrScrnFull: 0x4546,
  _RunIndicOff: 0x4570,
  _RunIndicOn: 0x456d,
  _DispHL: 0x4507,
  _GetCSC: 0x4018,
};

export default romCalls;