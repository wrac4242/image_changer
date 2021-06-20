# To Do list
basic way of seeing what needs to be done and how to structure it

### how it should be interacted with
```
use image_changer::img;

image = img::new(<path to image>); //creates the image

image.to_black_white(); //converts image to black and white

img::out_to_file(image); //outputs image to a file
```

## main todo list
+ add some basic filters to the code

+ add testing into the code, compare file hashes i guess?
+ move filters into their own closures and pass into a main function
+ change all pixels to the closest of some given pixels
+ custom filter, allow giving a closure

### general questions
+ how can we test the whole thing


#### finished from todo list
+ ~~add functionality to load a specified image via its path~~
+ ~~add functionality to output the image to a specified file~~  

+ ~~add multiple filetype functionality - image libary already supports~~

+ ~~how do we deal with images in memory? use the image library?~~ - images library

~~+ move the code in main.rs into testing~~
