# hex2ascii

Simple binary program to convert hex value of color to nearest ascii color (256 colors)

## Installation 
```bash
git clone https://github.com/Dartt0n/hex2ascii.git
cd hex2ascii
cargo install --path .
```

## Usage
### Simple usage
```bash
hex2ascii FF4D00
```
### Or can be combined with other programs
```bash
cxxmatrix --color=$(hex2ascii ff4d00) --frame-rate=60 --scene="rain-forever"
``` 
cxxmatrix: https://github.com/akinomyoga/cxxmatrix
