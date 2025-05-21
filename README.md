# BaseCodec 0.1.5

A highly extensible and customizable Base64 encoder/decoder that allows redefining the encoding character set, specific algorithms for each encoding group, etc. 

### Usage: Base64 [Command Options] [Text] 

#### **Command Options:** 

Base64_v0.1.5 [Option] [Text] [Codec]

Option:
- e|-e      Encoding mode
- d|-d      Decoding mode
- fe|-fe    [Read from file] Encoding mode 
- fd|-fd    [Read from file] Decoding mode
- fef|-fef  [Read from file, Output to file] Encoding, especially when existing special characters. 
- fdf|-fdf  [Read from file, Output to file] Decoding, especially when existing special characters. 

Codec:
- std|-std  Use the Base64 standard character set and standard algorithm. [Default]
- web|-web  Use the Base64 Web character set and standard algorithm, known as the Web variant algorithm.

#### More info

The 0.1.4 version of the project has also been merged into ExplorerSec/RustHttpServer as a base library.

# BaseCodec 0.1.5

一个高度可扩展可自定义 Base64 编码解码器，允许重新定义编码字符集、每个编码分组的具体算法等。

### 使用方式： Base64 [命令选项] [文本] 

#### **命令选项:**

Base64_v0.1.5 [Option] [Text] [Codec]

Option:
- e|-e      编码模式
- d|-d      解码模式
- fe|-fe    [从文件读入] 编码模式
- fd|-fd    [从文件读入] 解码模式
- fef|-fef  [从文件读入, 输出到文件] 编码模式, 尤其用于存在不能正常显示到终端的字符时。
- fdf|-fdf  [从文件读入, 输出到文件] 解码模式，尤其用于存在不能正常显示到终端的字符时。 

Codec:
- std|-std  采用 Base64 标准字符集和标准算法。[默认] 
- web|-web  采用 Base64 Web 字符集和标准算法，即所谓的 Web 变种算法。

#### 更多信息

该项目自 0.1.4 版本也被合并到 ExplorerSec/RustHttpServer 中，作为基础库使用。
