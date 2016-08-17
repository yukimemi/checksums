initSidebarItems({"fn":[["add_bytes_to_bits","Adds the specified number of bytes to the bit count. panic!() if this would cause numeric overflow."],["add_bytes_to_bits_tuple","Adds the specified number of bytes to the bit count, which is a tuple where the first element is the high order value. panic!() if this would cause numeric overflow."],["copy_memory","Copy bytes from src to dest"],["read_u32_be","Read the value of a vector of bytes as a u32 value in big-endian format."],["read_u32_le","Read the value of a vector of bytes as a u32 value in little-endian format."],["read_u32v_be","Read a vector of bytes into a vector of u32s. The values are read in big-endian format."],["read_u32v_le","Read a vector of bytes into a vector of u32s. The values are read in little-endian format."],["read_u64v_be","Read a vector of bytes into a vector of u64s. The values are read in big-endian format."],["read_u64v_le","Read a vector of bytes into a vector of u64s. The values are read in little-endian format."],["symm_enc_or_dec","symm_enc_or_dec() implements the necessary functionality to turn a SynchronousStreamCipher into an Encryptor or Decryptor"],["write_u32_be","Write a u32 into a vector, which must be 4 bytes long. The value is written in big-endian format."],["write_u32_le","Write a u32 into a vector, which must be 4 bytes long. The value is written in little-endian format."],["write_u32v_le","Write a vector of u32s into a vector of bytes. The values are written in little-endian format."],["write_u64_be","Write a u64 into a vector, which must be 8 bytes long. The value is written in big-endian format."],["write_u64_le","Write a u64 into a vector, which must be 8 bytes long. The value is written in little-endian format."],["write_u64v_le","Write a vector of u64s into a vector of bytes. The values are written in little-endian format."],["xor_keystream","XOR plaintext and keystream, storing the result in dst."],["zero","Zero all bytes in dst"]],"struct":[["FixedBuffer128","A fixed size buffer of 128 bytes useful for cryptographic operations."],["FixedBuffer64","A fixed size buffer of 64 bytes useful for cryptographic operations."]],"trait":[["FixedBuffer","A FixedBuffer, likes its name implies, is a fixed size buffer. When the buffer becomes full, it must be processed. The input() method takes care of processing and then clearing the buffer automatically. However, other methods do not and require the caller to process the buffer. Any method that modifies the buffer directory or provides the caller with bytes that can be modifies results in those bytes being marked as used by the buffer."],["StandardPadding","The StandardPadding trait adds a method useful for various hash algorithms to a FixedBuffer struct."],["WriteExt","An extension trait to implement a few useful serialization methods on types that implement Write"]]});