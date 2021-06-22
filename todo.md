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
- replace all of colour 1 with colour 2
- black white to a different colour
- thermal like camera
- background filter
- blur filter

+ add information commands to the code

+ add a way to run 3x3 filters on it
- closure that takes in an array of 9 pixels, allows doing stuff to said array
+ add testing into the code, compare file hashes i guess?
+ change all pixels to the closest of some given pixels
+ custom filter, allow giving a closure
+ copy parts of the image
+ allow image overlaying at set coordinates
+ do a function on a set area of the image
