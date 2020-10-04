Motion Detector
===============
A project made for Auburn ELEC 7450: Digtal Image Processing. The source code can be found on my `Github`_

.. _GitHub: https://github.com/joeyahines/motion_detector

Goal
++++
The goal of this project was to detect motion on a video stream from a `VideoForLinux`_ source. The algorithm
can also be tested by loading in individual frames.

.. _VideoForLinux: https://en.wikipedia.org/wiki/Video4Linux

Implementation
++++++++++++++
The project was written in C and VideoForLinux for grabbing image data and `SDL`_ for rendering the video
output. A background model is built by implementing a moving average of the image. In addition to this,
a motion mask is implemented to desensitise the algorithm from background motion objects.

.. _SDL: https://www.libsdl.org/

Webcam output w/ motion highlighted:

.. image:: https://github.com/joeyahines/motion_detector/blob/master/docs/final_report/motion.png?&raw=true
    :width: 60%
    :height: auto

Motion detection layer:

.. image:: https://github.com/joeyahines/motion_detector/blob/master/docs/final_report/motion_image.png?raw=true
    :width: 60%
    :height: auto

