``` 
 ______     ______     ______     __     __     ______   __  __    
/\  __ \   /\  ___\   /\  ___\   /\ \   /\ \   /\  ___\ /\ \_\ \   
\ \  __ \  \ \___  \  \ \ \____  \ \ \  \ \ \  \ \  __\ \ \____ \  
 \ \_\ \_\  \/\_____\  \ \_____\  \ \_\  \ \_\  \ \_\    \/\_____\ 
  \/_/\/_/   \/_____/   \/_____/   \/_/   \/_/   \/_/     \/_____/ 
                                                                   
 ```

A commandline tool that converts images and gifs into ascii art that can be printed to the 
terminal or save as a new file. 

Supported file formats:
* JPEG/JPG
* PNG
* GIF

## Download locally

    1. Clone repo.
    2. cd into project director of download. 
    3. To build executable run `cargo build --release`

# Basic usage
Simply follow the command with the path the the files you want converted, you can supply multiple files.
```
asciify <image file paths>
```

# Flags

### --color or -c
Allows to display images/gifs with their original color
> **Note:** To use this feature you terminal appication MUST support 24-bit or 8-bit colors, coloring will default to
> [truecolor](https://gist.github.com/CMCDragonkai/146100155ecd79c7dac19a9e23e6a362) if availble else 8-bit ansi color
> codes will be used.
```
asciify <image file paths> --color

asciify <image file paths> -c
```

### --detailed or -d
Allows pixel output intensity to be encoded with a wider range of characters (70 as opposed to the default 10)
```
asciify <image file paths> --detailed

asciify <image file paths> -d
```

### --mapping or -m
User can provide custom character string which will be used to encode the pixel intensities of the resulting image.
```
asciify <image file paths> --mapping <custom string>

asciify <image file paths> -m <custom string>
```

### --scale-factor or -s
> **Note:** Preserves size ratio of original image. Scale variable is only applied to the longest 
> out of height and width.
```
asciify <image file paths> --scale_factor <integer scale>

asciify <image file paths> -m <integer scale>
```

### --help -h
Pulls up directions for use.
```
asciify <image file paths> --help

asciify <image file paths> -h
```

### --version -V
Get the current app version.
```
asciify <image file paths> --version

asciify <image file paths> -V
```
