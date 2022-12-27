# This Website (V3)

This is the 3rd iteration of this website. The 1st iteration was a simple HTML page built with bootstrap
that looked similar to the current design. The 2nd iteration moved to Rust and the Rocket framework and
was more dynamic allowing for RST pages to be rendered to html. This latest iteration uses Axum and Markdown
rendered to HTML for the dynamic content.

## Moving away from Rocket
Rocket is a great framework and will probably be the Rust goto. However, it is very complex and comes
with a lot of bells and whistles I don't need. Its also in active development which meant it became very
hard to update the site. Everytime I wanted to add a new feature I would go to update Rocket and have something
break. 

I originally tried Warp, which I have used for some other projects. While I like it mostly, I find it very
hard to move past the basic examples. And when you run into issues with it, the compiler loves to spit out
very esoteric errors that are a pain to debug. 

I'm also a low level developer. I like working at lower levels of interfaces. Axum is pretty high level
in the grand scheme of things, but it doesn't seem overly opinionated in its design. Axum has its own issues
and I doubt this will be the last time I switch the framework...

## Moving away from RST
I lied, in the last updated for this site I said how much I loved RST. After using more Markdown for note-taking
and for some documentation, I realized I like it more. I always decided to not write my own parse for it.
Rust has some good libraries that render MD to HTML. 

# This Website (V2)
> NOTE: This is the old version of this page for the second version of this site. It's here for archival purposes.
This was originally written in RST and has been converted to MD for v3. 

## This Website
This is the 2nd version of my personal website, the first version was very similar to this but written in plain HTML
using Bootstrap. I was never very happy with how it looked and it being raw HTML limited what I could do with it. To
improve it, I set out with a few goals:

* Use a web framework to improve capabilities
* Be easy to deploy
* Allow things, like the resume, to be easily editable
* Improve aesthetics
* Ensure mobile version also looks good

### Picking Rust and Rocket
As I am not much of a UI person, the website update was put on the back burner for sometime. The old site was "good
enough." I originally considered doing it in Python and using Django. But that fails the criteria of being easily
deployable. In early 2020, I began learning Rust out of curiosity and the need to use it for research.
I decided to tackle the website backend in Rust as a learning exercise. I went with the Rocket framework. It
seemed to suit my needs and supported Tera templates. Tera templates are very similar to Django's templates that I
had already had experience in.

### Easily Editable
As the style of the site is a simplistic linux terminal, I decided it would be nice to have the longer pages be in
raw reStructuredText. This fits the aesthetic of the site well and unlike HTML files, they are raw text and are easy to
write and read while editing them. RST files can also easily be built into PDFs, which is great for generating a resume.

To create new pages, I simply have to add new .rst files to a directory on the website. When the templates are rendered,
the Rust backend looks for all pages and lists them. To control ordering, I implemented a ranking system where the first
character of a file name is its rank. For example, my resume is in a file called 1resume.rst. This gives it rank of
1 so it shows up on the main page before other links.

### Improving Aesthetics
Like stated earlier, I am no UI designer. The terminal style meant I could put very little effort into the design of the
actual website. Somehow in version one, I still managed to mess that up. This was mainly due to hacking together
bootstrap to give me something that looked like a terminal. This time, I dumped Bootstrap and wrote my own CSS. I should
have done that the first place as it was easy to do and produced a far better result. This also has the side effect of
improving how the site looked on mobile.

Future Improvements
-------------------
As I continue to learn Rust, I plan to implement more features of RST into the raw file displays. ~~For example,
embedding images~~ (now onto text formatting...). I want to strike a balance between the aesthetic of a terminal and
ease of use of a website.
