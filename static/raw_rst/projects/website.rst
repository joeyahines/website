This Website
============
This is the 2nd version of my personal website, the first version was very similar to this but written in plain HTML
using Bootstrap. I was never very happy with how it looked and it being raw HTML limited what I could do with it. To
improve it, I set out with a few goals:

* Use a web framework to improve capabilities
* Be easy to deploy
* Allow things, like the resume, to be easily editable
* Improve aesthetics
* Ensure mobile version also looks good

Picking Rust and Rocket
-----------------------
As I am not much of a UI person, the website update was put on the back burner for sometime. The old site was "good 
enough." I originally considered doing it in Python and using Django. But that fails the criteria of being easily
deployable. In early 2020, I began learning Rust out of curiosity and the need to use it for research. 
I decided to tackle the website backend in Rust as a learning exercise. I went with the `Rocket`_ framework. It
seemed to suit my needs and supported Tera templates. Tera templates are very similar to Django's templates that I
had already had experience in.

.. _Rocket: https://rocket.rs/

Easily Editable
---------------
As the style of the site is a simplistic linux terminal, I decided it would be nice to have the longer pages be in
raw reStructuredText. This fits the aesthetic of the site well and unlike HTML files, they are raw text and are easy to
write and read while editing them. RST files can also easily be built into PDFs, which is great for generating a resume.

To create new pages, I simply have to add new .rst files to a directory on the website. When the templates are rendered,
the Rust backend looks for all pages and lists them. To control ordering, I implemented a ranking system where the first
character of a file name is its rank. For example, my resume is in a file called 1resume.rst. This gives it rank of
1 so it shows up on the main page before other links.

Improving Aesthetics
--------------------
Like stated earlier, I am no UI designer. The terminal style meant I could put very little effort into the design of the
actual website. Somehow in version one, I still managed to mess that up. This was mainly due to hacking together
bootstrap to give me something that looked like a terminal. This time, I dumped Bootstrap and wrote my own CSS. I should
have done that the first place as it was easy to do and produced a far better result. This also has the side effect of
improving how the site looked on mobile.

Future Improvements
-------------------
As I continue to learn Rust, I plan to implement more features of RST into the raw file displays. For example,
embedding images. I want to strike a balance between the aesthetic of a terminal and ease of use of a website.
