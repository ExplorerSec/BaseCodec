use std::collections::HashMap;

pub struct CodecTable {
    padding: char,
    table: Vec<u8>, // len == 64
    d_table: HashMap<char, u8>,
}

impl CodecTable {
    pub fn new(s: &str, ch: char) -> CodecTable {
        let mut dtb = HashMap::new();
        let mut idx = 0;
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            dtb.insert(c, idx);
            idx += 1;
        }
        assert_eq!(dtb.len(), 64);
        dtb.insert(ch, 0);
        CodecTable {
            padding: ch,
            table: s.as_bytes().to_vec(),
            d_table: dtb,
        }
    }
    pub fn default() -> CodecTable {
        let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        Self::new(s, '=')
    }
    pub fn encode(&self, s: &str) -> String {
        let mut vec: Vec<u8> = s.as_bytes().to_vec();
        let mut num_padding = 0;
        while vec.len() % 3 != 0 {
            vec.push(0);
            num_padding += 1;
        }
        let len = vec.len() / 3;
        let mut v_out: Vec<u8> = Vec::with_capacity(len * 4);
        for i in 0..len {
            v_out.push(self.table[(vec[i * 3] >> 2) as usize]);
            v_out.push(self.table[(((vec[i * 3] & 0b11) << 4) | (vec[i * 3 + 1] >> 4)) as usize]);
            v_out.push(self.table[(((vec[i * 3 + 1] << 4) >> 2) | (vec[i * 3 + 2] >> 6)) as usize]);
            v_out.push(self.table[((vec[i * 3 + 2] << 2) >> 2) as usize]);
        }
        let mut s_out = String::from_utf8(v_out).unwrap();
        for _ in 0..num_padding {
            s_out.pop();
        }
        for _ in 0..num_padding {
            s_out.push(self.padding);
        }

        s_out
    }
    pub fn decode(&self, s: &str) -> String {
        let mut vec: Vec<u8> = Vec::new();
        let mut vec_out: Vec<u8> = Vec::new();
        let mut num_padding = 0;
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == self.padding {
                num_padding += 1;
            }
            vec.push(self.d_table[&c]);
            if vec.len() == 4 {
                vec_out.push((vec[0] << 2) | ((vec[1]) >> 4));
                vec_out.push((vec[1] << 4) | (vec[2] >> 2));
                vec_out.push((vec[2] << 6) | vec[3]);
                vec.clear();
            }
        }

        match String::from_utf8(vec_out) {
            Ok(mut s) => {
                while num_padding > 0 {
                    s.pop();
                    num_padding -= 1;
                }
                s
            }
            _ => String::from("[Error] Invalid Text!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CodecTable;
    #[test]
    fn test1_default() {
        let ct = CodecTable::default();
        assert_eq!(
            ct.table,
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes()
        );
        assert_eq!(ct.padding, '=');
    }
    #[test]
    fn test2_en_utf8() {
        let ct = CodecTable::default();
        assert_eq!(
            ct.encode("This is a Base64 Test"),
            "VGhpcyBpcyBhIEJhc2U2NCBUZXN0"
        );
        assert_eq!(
            ct.encode("这是一个进行编码的测试"),
            "6L+Z5piv5LiA5Liq6L+b6KGM57yW56CB55qE5rWL6K+V"
        );
    }
    #[test]
    fn test3_en_utf8_padding() {
        let ct = CodecTable::default();
        assert_eq!(ct.encode("12345"), "MTIzNDU=");
        assert_eq!(ct.encode("padding"), "cGFkZGluZw==");
    }
    #[test]
    fn test4_de_utf8() {
        let ct = CodecTable::default();
        assert_eq!(
            ct.decode("6L+b6KGM6Kej56CB5rWL6K+V").as_bytes(),
            "进行解码测试".as_bytes()
        );
        assert_eq!(
            ct.decode("WlhDVkJOTUxLSkhHRkRTQVFXRVJUWVVJT1A7").as_bytes(),
            "ZXCVBNMLKJHGFDSAQWERTYUIOP;".as_bytes()
        );
    }
    #[test]
    fn test4_de_utf8_padding() {
        let ct = CodecTable::default();
        assert_eq!(ct.decode("QUJDRA==").as_bytes(), "ABCD".as_bytes());
    }
    #[test]
    fn test5_en_de() {
        let ct = CodecTable::default();
        let s = "Base64 是一种基于 64 个可打印字符来表示二进制数据的表示方法，由于 2^6=64，所以每 6 个比特为一个单元，对应某个可打印字符。";
        assert_eq!(ct.decode(&ct.encode(s)).as_bytes(), s.as_bytes());
    }
}
