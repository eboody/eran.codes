## 👁️ CSS Scope Inline vs Tailwind CSS Showdowns
### Basics
Tailwind verbosity goes up with more child elements.
```html
<!-- CSS Scope Inline -->
<div>
    <style>
        me { background: red; }
        me div { background: green; }
        me [n1] { background: yellow; }
        me [n2] { background: blue; }
    </style>
    red
    <div>green</div>
    <div>green</div>
    <div>green</div>
    <div n1>yellow</div>
    <div n2>blue</div>
    <div>green</div>
    <div>green</div>
</div>

<!-- Tailwind -->
<div class="bg-[red]">
    red
    <div class="bg-[green]">green</div>
    <div class="bg-[green]">green</div>
    <div class="bg-[green]">green</div>
    <div class="bg-[yellow]">yellow</div>
    <div class="bg-[blue]">blue</div>
    <div class="bg-[green]">green</div>
    <div class="bg-[green]">green</div>
</div>
```

### CSS variables and child elements
At first glance, **Tailwind Example 2** looks very promising! Exciting ...but:
* 🔴 **Every child style requires an explicit selector.**
  * Tailwinds' shorthand advantages sadly disappear.
  * Any more child styles added in Tailwind will become longer than vanilla CSS.
  * This limited example is the best case scenario for Tailwind.
* 🔴 Not visible on github: **no highlighting for properties and units** begins to be painful.
```html
<!doctype html>
<html>
    <head>
        <style>
            :root {
                --color-1: hsl(0 0% 88%);
                --color-1-active: hsl(214 20% 70%);
            }
        </style>
        <script src="https://cdn.tailwindcss.com"></script>
        <script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js"></script>
    </head>
    <body>
        <!-- CSS Scope Inline -->
        <div>
            <style>
               me { margin:8px 6px; }
               me div a { display:block; padding:8px 12px; margin:10px 0; background:var(--color-1); border-radius:10px; text-align:center; }
               me div a:hover { background:var(--color-1-active); color:white; }
            </style>
            <div><a href="#">Home</a></div>
            <div><a href="#">Team</a></div>
            <div><a href="#">Profile</a></div>
            <div><a href="#">Settings</a></div>
            <div><a href="#">Log Out</a></div>
        </div>

        <!-- Tailwind Example 1 -->
        <div class="mx-2 my-4">
            <div><a href="#" class="block py-2 px-3 my-2 bg-[--color-1] rounded-lg text-center hover:bg-[--color-1-active] hover:text-white">Home</a></div>
            <div><a href="#" class="block py-2 px-3 my-2 bg-[--color-1] rounded-lg text-center hover:bg-[--color-1-active] hover:text-white">Team</a></div>
            <div><a href="#" class="block py-2 px-3 my-2 bg-[--color-1] rounded-lg text-center hover:bg-[--color-1-active] hover:text-white">Profile</a></div>
            <div><a href="#" class="block py-2 px-3 my-2 bg-[--color-1] rounded-lg text-center hover:bg-[--color-1-active] hover:text-white">Settings</a></div>
            <div><a href="#" class="block py-2 px-3 my-2 bg-[--color-1] rounded-lg text-center hover:bg-[--color-1-active] hover:text-white">Log Out</a></div>
        </div>

        <!-- Tailwind Example 2 -->
        <div class="mx-2 my-4
            [&_div_a]:block [&_div_a]:py-2 [&_div_a]:px-3 [&_div_a]:my-2 [&_div_a]:bg-[--color-1] [&_div_a]:rounded-lg [&_div_a]:text-center
            [&_div_a:hover]:bg-[--color-1-active] [&_div_a:hover]:text-white">
            <div><a href="#">Home</a></div>
            <div><a href="#">Team</a></div>
            <div><a href="#">Profile</a></div>
            <div><a href="#">Settings</a></div>
            <div><a href="#">Log Out</a></div>
        </div>
    </body>
</html>
```
