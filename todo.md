# To Do list
basic way of seeing what needs to be done and how to structure it

### how it should be interacted with
```
use image_changer::Img;

image = Img::new(<path to image>); //creates the image

image.to_black_white(); //converts image to black and white

Img::out_to_file(image); //outputs image to a file
```

## main todo list
+ add functionality to load a specified image via its path
+ add functionality to output the image to a specified file  
    which if isnt set, just default it to something else

+ add multiple filetype functionality - image libary already supports
+ add some basic filters to the code

### general questions
+ how do we deal with images in memory? use the image library?
+ how can we test the whole thing?
+ how do we deal with different output types
