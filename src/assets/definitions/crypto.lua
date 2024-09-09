---@meta
---@diagnostic disable: missing-return, unused-local

---Crypto object used to encode and decode ASCII text strings to and from Base64, as well as obtain CRC16, HMAC, and MD5 values for specified strings.
---@class Crypto

Crypto = {}

---Compute the Base64 of a specified string.
---
---@param value string The value, enclosed within quotes, to encode to Base64
---@param pad? boolean Replace with true | false. If true, output is padded with '=' signs. The default is true.
---@return string encoded The encoded value
function Crypto.Base64Encode(value, pad) end

---Convert the Base64 of a specified value to a string.
---
---@param value string The value, enclosed within quotes, to decode from Base64.
---@return string decoded The decoded value
function Crypto.Base64Decode(value) end

---Compute the CRC16 of a specified value.
---
---@param value string The value, enclosed within quotes, used to generate the CRC16.
---@return string encrypted
function Crypto.CRC16Compute(value) end

---@class Crypto.Hash
Crypto.Hash = {}
---@type Crypto.Hash
Crypto.Hash.MD5 = {}
---@type Crypto.Hash
Crypto.Hash.SHA1 = {}
---@type Crypto.Hash
Crypto.Hash.SHA256 = {}
---@type Crypto.Hash
Crypto.Hash.SHA512 = {}

---@class Crypto.Cipher
Crypto.Cipher = {}

--- AES 128 CBC encryption
---@type Crypto.Cipher
Crypto.Cipher.AES_128_CBC = {}

--- AES 128 ECB encryption
---@type Crypto.Cipher
Crypto.Cipher.AES_128_ECB = {}

--- AES 256 CBC encryption
---@type Crypto.Cipher
Crypto.Cipher.AES_256_CBC = {}

--- AES 256 ECB encryption
---@type Crypto.Cipher
Crypto.Cipher.AES_256_ECB = {}

---Block-based decryption. Will raise error on failure.
---Note: Crypto.Decrypt requires that the key and iv match the length of the cipher
---@param cipher Crypto.Cipher One of the Crypto.Cipher algorithm options.
---@param key string
---@param iv string | nil
---@param cipher_text string
---@return string decrypted
function Crypto.Decrypt(cipher, key, iv, cipher_text) end

---Block-based decryption. Will raise error on failure.
---Note: Crypto.Encrypt requires that the key and iv match the length of the cipher
---@param cipher Crypto.Cipher The cipher algorithm to use
---@param key string
---@param iv string | nil
---@param message string
---@param padding boolean default = true, if false then message must be multiple of block size
---@return string decrypted
function Crypto.Encrypt(cipher, key, iv, message, padding) end

---Compute the message digest of specified data using a specified hashing algorithm.
---@param algorithm Crypto.Hash The cipher algorithm to use
---@param data string The data for which to compute the message authentication code, enclosed within quotes.
---@return string
function Crypto.Digest(algorithm, data) end

---Create a series of random bytes.
---@param count number
---@return string bytes
function Crypto.GetRandomBytes(count) end

---Compute the message authentication code of specified data using a specified hashing algorithm and key.
---
---@param algorithm Crypto.Hash The cipher algorithm to use.
---@param key string The secret key to use for computing the message authentication code, enclosed within quotes.
---@param data string The data for which to compute the message authentication code, enclosed within quotes.
---@return string message_authentication
function Crypto.HMAC(algorithm, key, data) end

---Compute the MD5 hash of a specified value.
---!PLEASE NOTE THAT FOR MODERN SYSTEM MD5 ENCRYPTION IS NO MORE SECURE!
---@param value string The value for which to compute the MD5 hash.
---@return string
function Crypto.MD5Compute(value) end

---Implementation of Password-Based Key Derivation Function 2. Will raise error on failure.
---@param password string
---@param salt string
---@param iterations number
---@param key_length number
---@param digest Crypto.Hash
---@return string
function Crypto.PBKDF2(password, salt, iterations, key_length, digest) end
