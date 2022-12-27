# Motion Detector
A project made for Auburn ELEC 7450: Digtal Image Processing. The source code can be found on my
[GitHub](https://github.com/joeyahines/motion_detector)

## Goal
The goal of this project was to detect motion on a video stream from a [VideoForLinux](https://en.wikipedia.org/wiki/Video4Linux) source. 
The algorithm can also be tested by loading in individual frames.

## Implementation
The project was written in C and VideoForLinux for grabbing image data and [SDL](https://www.libsdl.org/)
for rendering the video  output. A background model is built by implementing a moving average of the image. 
In addition to this,  a motion mask is implemented to desensitise the algorithm from background motion objects.

Webcam output w/ motion highlighted:

![motion capture](https://github.com/joeyahines/motion_detector/blob/master/docs/final_report/motion.png?&raw=true)

Motion detection layer:

![motion detection layer](https://github.com/joeyahines/motion_detector/blob/master/docs/final_report/motion_image.png?raw=true)

