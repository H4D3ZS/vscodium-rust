import { describe, it, expect } from 'vitest';

// Extracted from jetskiAgentShim.js to run in Vitest environment
function encodeConnectChunk(payload: any, isEos = false): Uint8Array {
  const encoder = new TextEncoder();
  const bytes = encoder.encode(JSON.stringify(payload));
  const header = new Uint8Array(5);
  header[0] = isEos ? 0x02 : 0x00; // Flag: 0x00 = Message, 0x02 = EOS Metadata
  const len = bytes.length;
  header[1] = (len >> 24) & 0xff;
  header[2] = (len >> 16) & 0xff;
  header[3] = (len >> 8) & 0xff;
  header[4] = len & 0xff;

  const chunk = new Uint8Array(5 + len);
  chunk.set(header, 0);
  chunk.set(bytes, 5);
  return chunk;
}

describe('ConnectRPC Envelope Serialization', () => {
  it('should encode a normal message chunk with 0x00 flag and correct length', () => {
    const payload = { text: "hello world" };
    const chunk = encodeConnectChunk(payload, false);
    
    // First byte must be 0x00 (Message)
    expect(chunk[0]).toBe(0x00);
    
    // Remaining bytes are the length of the stringified payload
    const jsonStr = JSON.stringify(payload);
    const expectedLength = new TextEncoder().encode(jsonStr).length;
    
    // Read big-endian uint32 length from header bytes 1 to 4
    const lengthFromHeader = (chunk[1] << 24) | (chunk[2] << 16) | (chunk[3] << 8) | chunk[4];
    expect(lengthFromHeader).toBe(expectedLength);
    expect(chunk.length).toBe(5 + expectedLength);
    
    // Decode the body to verify payload matches
    const bodyBytes = chunk.subarray(5);
    const decodedPayload = JSON.parse(new TextDecoder().decode(bodyBytes));
    expect(decodedPayload).toEqual(payload);
  });

  it('should encode an EOS metadata chunk with 0x02 flag and empty body length', () => {
    const payload = {};
    const chunk = encodeConnectChunk(payload, true);
    
    // First byte must be 0x02 (EOS)
    expect(chunk[0]).toBe(0x02);
    
    const jsonStr = JSON.stringify(payload);
    const expectedLength = new TextEncoder().encode(jsonStr).length;
    
    const lengthFromHeader = (chunk[1] << 24) | (chunk[2] << 16) | (chunk[3] << 8) | chunk[4];
    expect(lengthFromHeader).toBe(expectedLength);
    expect(chunk.length).toBe(5 + expectedLength);
  });
});
