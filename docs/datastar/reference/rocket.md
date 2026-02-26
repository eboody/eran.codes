# Rocket

- Source: `https://data-star.dev/reference/rocket`
- Retrieved: `2026-02-26 17:55 UTC`
- Section: Reference

Rocket is currently in alpha – available in the Datastar Pro repo.

Rocket is a [Datastar Pro](/datastar_pro) plugin that bridges [Web Components](https://developer.mozilla.org/en-US/docs/Web/API/Web_components) with Datastar’s reactive system. It allows you to create encapsulated, reusable components with reactive data binding.

> Rocket is a powerful feature, and should be used sparingly. For most applications, standard Datastar templates and global signals are sufficient. Reserve Rocket for cases where component encapsulation is essential, such as integrating third-party libraries or creating complex, reusable UI elements.

### Basic example 

Traditional web components require verbose class definitions and manual DOM management. Rocket eliminates this complexity with a declarative, template-based approach.

Here’s a Rocket component compared to a vanilla web component.
    
    
    <template data-rocket:simple-counter
              data-props:count="int|min:0|=0"
              data-props:start="int|min:0|=0"
              data-props:step="int|min:1|max:10|=1"
    >
      <script>
        $$count = $$start
      </script>
      <template data-if="$$errs?.start">
        <div data-text="$$errs.start[0].value"></div>
      </template>
      <template data-if="$$errs?.step">
        <div data-text="$$errs.step[0].value"></div>
      </template>
      <button data-on:click="$$count -= $$step">-</button>
      <span data-text="$$count"></span>
      <button data-on:click="$$count += $$step">+</button>
      <button data-on:click="$$count = $$start">Reset</button>
    </template>
    
    
    class SimpleCounter extends HTMLElement {
      static observedAttributes = ['start', 'step'];
      
      constructor() {
        super();
        this.innerHTML = `
          <div class="error" style="display: none;"></div>
          <button class="dec">-</button>
          <span class="count">0</span>
          <button class="inc">+</button>
          <button class="reset">Reset</button>
        `;
        
        this.errorEl = this.querySelector('.error');
        this.decBtn = this.querySelector('.dec');
        this.incBtn = this.querySelector('.inc');
        this.resetBtn = this.querySelector('.reset');
        this.countEl = this.querySelector('.count');
        
        this.handleDec = () => { 
          const newValue = this.count - this.step;
          if (newValue >= 0) {
            this.count = newValue;
            this.updateDisplay();
          }
        };
        this.handleInc = () => { 
          this.count += this.step;
          this.updateDisplay();
        };
        this.handleReset = () => { 
          this.count = this.start; 
          this.updateDisplay(); 
        };
        
        this.decBtn.addEventListener('click', this.handleDec);
        this.incBtn.addEventListener('click', this.handleInc);
        this.resetBtn.addEventListener('click', this.handleReset);
      }
      
      connectedCallback() {
        const startVal = parseInt(this.getAttribute('start') || '0');
        const stepVal = parseInt(this.getAttribute('step') || '1');
        
        if (startVal < 0) {
          this.errorEl.textContent = 'start must be at least 0';
          this.errorEl.style.display = 'block';
          this.start = 0;
        } else {
          this.start = startVal;
          this.errorEl.style.display = 'none';
        }
        
        if (stepVal < 1 || stepVal > 10) {
          this.errorEl.textContent = 'step must be between 1 and 10';
          this.errorEl.style.display = 'block';
          this.step = Math.max(1, Math.min(10, stepVal));
        } else {
          this.step = stepVal;
          if (this.start === startVal) {
            this.errorEl.style.display = 'none';
          }
        }
        
        this.count = this.start;
        this.updateDisplay();
      }
      
      disconnectedCallback() {
        this.decBtn.removeEventListener('click', this.handleDec);
        this.incBtn.removeEventListener('click', this.handleInc);
        this.resetBtn.removeEventListener('click', this.handleReset);
      }
      
      attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'start') {
          const startVal = parseInt(newValue || '0');
          if (startVal < 0) {
            this.errorEl.textContent = 'start must be at least 0';
            this.errorEl.style.display = 'block';
            this.start = 0;
          } else {
            this.start = startVal;
            this.errorEl.style.display = 'none';
          }
          this.count = this.start;
        } else if (name === 'step') {
          const stepVal = parseInt(newValue || '1');
          if (stepVal < 1 || stepVal > 10) {
            this.errorEl.textContent = 'step must be between 1 and 10';
            this.errorEl.style.display = 'block';
            this.step = Math.max(1, Math.min(10, stepVal));
          } else {
            this.step = stepVal;
            this.errorEl.style.display = 'none';
          }
        }
        if (this.isConnected) {
          this.updateDisplay();
        }
      }
      
      updateDisplay() {
        this.countEl.textContent = this.count;
      }
    }
    
    customElements.define('simple-counter', SimpleCounter);

## Overview 

Rocket allows you to turn HTML templates into fully reactive web components. The backend remains the source of truth, but your frontend components are now encapsulated and reusable without any of the usual hassle.

Add `data-rocket:my-component` to a `template` element to turn it into a Rocket component. Component signals are automatically scoped with `$$`, so component instances don’t interfere with each other.

You can use Rocket to wrap external libraries using module imports, and create references to elements within your component. Each component gets its own signal namespace that plays nicely with Datastar’s global signals. When you remove a component from the DOM, all its `$$` signals are cleaned up automatically.

### Bridging Web Components and Datastar 

Web components want encapsulation; Datastar wants a global signal store. Rocket gives you both by creating isolated namespaces for each component. Each instance gets its own sandbox that doesn’t mess with other components on the page, or with global signals.

Multiple component instances work seamlessly, each getting its own numbered namespace. You still have access to global signals when you need them, but your component state stays isolated and clean.

### Signal Scoping 

Use `$$` for component-scoped signals, and `$` for global signals. Component signals are automatically cleaned up when you remove the component from the DOM - no memory leaks, no manual cleanup required.

Behind the scenes, your `$$count` becomes something like `$._rocket.my_counter.id1.count`, with each instance getting its own id-prefixed namespace. You never have to think about this complexity - just write `$$count` and Rocket handles the rest.
    
    
    // Your component template writes:
    <button data-on:click="$$count++">Increment</button>
    <span data-text="$$count"></span>
    
    // Rocket transforms it to (for instance #1):
    <button data-on:click="$._rocket.my_counter.id1.count++">Increment</button>
    <span data-text="$._rocket.my_counter.id1.count"></span>
    
    // The global Datastar signal structure:
    $._rocket = {
      my_counter: {
        id1: { count: 0 }, // First counter instance
        id2: { count: 5 }, // Second counter instance
        id3: { count: 10 } // Third counter instance
      },
      user_card: {
        id4: { name: "Alice" }, // Different component type
        id5: { name: "Bob" }
      }
    }

## Defining Rocket Components 

Rocket components are defined using a HTML `template` element with the `data-rocket:my-component` attribute, where `my-component` is the name of the resulting web component. The name must contain at least one hyphen, as per the [custom element](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements#name) specification.
    
    
    <template data-rocket:my-counter>
      <script>
        $$count = 0  
      </script>
      <button data-on:click="$$count++">
        Count: <span data-text="$$count"></span>
      </button>
    </template>

This gets compiled to a web component, meaning that usage is simply:
    
    
    <my-counter></my-counter>

Rocket components _must_ be defined before being used in the DOM.
    
    
    <!-- Template element must appear first in the DOM. -->
    <template data-rocket:my-counter></template>
    
    <my-counter></my-counter>

## Signal Management 

Rocket makes it possible to work with both component-scoped and global signals (global to the entire page).

### Component Signals 

Component-scoped signals use the `$$` prefix and are isolated to each component instance.
    
    
    <template data-rocket:isolated-counter>
      <script>
        // These are component-scoped – each instance has its own values
        $$count = 0
        $$step = 1
        $$maxCount = 10
        $$isAtMax = computed(() => $$count >= $$maxCount)
        
        // Component actions
        action({
          name: 'increment',
          apply() {
            if ($$count < $$maxCount) {
              $$count += $$step
            }
          },
        })
      </script>
      
      <div>
        <p>Count: <span data-text="$$count"></span></p>
        <p data-show="$$isAtMax" class="error">Maximum reached!</p>
        <button data-on:click="@increment()" data-attr:disabled="$$isAtMax">+</button>
      </div>
    </template>
    
    <!-- Multiple instances work independently -->
    <isolated-counter></isolated-counter>
    <isolated-counter></isolated-counter>

### Global Signals 

Global signals use the `$` prefix and are shared across the entire page.
    
    
    <template data-rocket:theme-toggle>
      <script>
        // Access global theme state
        if (!$theme) {
          $theme = 'light'
        }
        
        action({
          name: 'toggleTheme',
          apply() {
            $theme = $theme === 'light' ? 'dark' : 'light'
          },
        })
      </script>
      
      <button data-on:click="@toggleTheme()">
        <span data-text="$theme === 'light' ? '🌙' : '☀️'"></span>
        <span data-text="$theme === 'light' ? 'Dark Mode' : 'Light Mode'"></span>
      </button>
    </template>
    
    <!-- All instances share the same global theme -->
    <theme-toggle></theme-toggle>
    <theme-toggle></theme-toggle>

## Props 

The `data-props:*` attribute allows you to define component props with codecs for validation and defaults.
    
    
    <!-- Component definition with defaults -->
    <template data-rocket:progress-bar
              data-props:value="int|=0"
              data-props:max="int|=100" 
              data-props:color="string|=blue"
    >
      <script>
        $$percentage = computed(() => Math.round(($$value / $$max) * 100))
      </script>
      
      <div class="progress-container">
        <div class="progress-bar" 
            data-style="{
              width: $$percentage + '%',
              backgroundColor: $$color
            }">
        </div>
        <span data-text="$$percentage + '%'"></span>
      </div>
    </template>
    
    <!-- Usage -->
    <progress-bar data-attr:value="'75'" data-attr:color="'green'"></progress-bar>
    <progress-bar data-attr:value="'30'" data-attr:max="'50'"></progress-bar>

Rocket automatically transforms and validates values using the codecs defined in `data-props:*` attributes.

## Setup Scripts 

Setup scripts initialize component behavior and run when the component is created. Rocket supports both component (per-instance) and static (one-time) setup scripts.

### Component Setup Scripts 

Regular `<script>` tags run for each component instance.
    
    
    <template data-rocket:timer
              data-props:seconds="int|=0"
              data-props:running="boolean|=false"
              data-props:interval="int|=1000"
    >
      <script>
        $$minutes = computed(() => Math.floor($$seconds / 60))
        $$displayTime = computed(() => {
          const m = String($$minutes).padStart(2, '0')
          const s = String($$seconds % 60).padStart(2, '0')
          return m + ':' + s
        })
        
        let intervalId
        effect(() => {
          if ($$running) {
            intervalId = setInterval(() => $$seconds++, $$interval)
          } else {
            clearInterval(intervalId)
          }
        })
        
        // Cleanup when component is removed
        onCleanup(() => {
          clearInterval(intervalId)
        })
      </script>
      
      <div>
        <h2 data-text="$$displayTime"></h2>
        <button data-on:click="$$running = !$$running" 
                data-text="$$running ? 'Stop' : 'Start'">
        </button>
        <button data-on:click="$$seconds = 0">Reset</button>
    </div>
    </template>

### Host Element Access 

Rocket injects an `el` binding into every component setup script. It always points to the current custom element instance, even when you opt into Shadow DOM, so you can imperatively read attributes, toggle classes, or wire event listeners.
    
    
    <template data-rocket:focus-pill>
      <script>
        el.setAttribute('role', 'button')
        el.addEventListener('focus', () => el.classList.add('is-focused'))
        el.addEventListener('blur', () => el.classList.remove('is-focused'))
      </script>
      
      <span><slot></slot></span>
    </template>

Setup code executes inside an arrow function sandbox, so `this` has no meaning inside component scripts. Use `el` any time you need the host element—for example to call `el.shadowRoot`, `el.setAttribute`, or pass it into a third-party library.

### Static Setup Scripts 

Scripts with a `data-static` attribute only run once, when the component type is first registered. This is useful for shared constants or utilities.
    
    
    <template data-rocket:icon-button>
      <script data-static>
        const icons = {
          heart: '❤️',
          star: '⭐',
          thumbs: '👍',
          fire: '🔥'
        }
      </script>
      
      <script>
        $$icon = $$type || 'heart'
        $$emoji = computed(() => icons[$$icon] || '❓')
      </script>
      
      <button data-on:click="@click()">
        <span data-text="$$emoji"></span>
        <span data-text="$$label || 'Click me'"></span>
      </button>
    </template>

## Module Imports 

Rocket allows you to wrap external libraries, loading them before the component initializes and the setup script runs. Use `data-import:*` for modern ES modules, and add the `__iife` modifier (`data-import:foo__iife`) for legacy globals.

### ESM Imports 

The `data-import:*` attribute loads modern ES modules by default.
    
    
    <template data-rocket:qr-generator
              data-props:text="string|trim|required!|=Hello World"
              data-props:size="int|min:50|max:1000|=200"
              data-import:qr="https://cdn.jsdelivr.net/npm/qr-creator@1.0.0/+esm"
    >
      <script>
        $$errorText = ''
        
        effect(() => {
          // Check for validation errors first
          if ($$hasErrs) {
            const messages = []
            if ($$errs?.text) {
              messages.push('Text is required')
            }
            if ($$errs?.size) {
              messages.push('Size must be 50-1000px')
            }
            $$errorText = messages.join(', ') || 'Validation failed'
            return
          }
    
          if (!$$canvas) {
            return
          }
    
          if (!qr) {
            $$errorText = 'QR library not loaded'
            return
          }
          
          try {
            qr.render({
              text: $$text,
              size: $$size
            }, $$canvas)
            $$errorText = ''
          } catch (err) {
            $$errorText = 'QR generation failed'
          }
        })
      </script>
      
      <div data-style="{width: $$size + 'px', height: $$size + 'px'}">
        <template data-if="!$$errorText">
          <canvas data-ref="canvas" style="display: block;"></canvas>
        </template>
        <template data-else>
          <div data-text="$$errorText" class="error"></div>
        </template>
      </div>
    </template>

### IIFE Imports 

Add the `__iife` modifier for legacy libraries that expose globals. The library must expose a global variable that matches the alias you specify after `data-import:`.
    
    
    <template data-rocket:chart
              data-props:data="json|=[]"
              data-props:type="string|=line"
              data-import:chart__iife="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.js"
    >
      <script>
        let chartInstance
        
        effect(() => {
          if (!$$canvas || !chart || !$$data.length) {
            return
          }
    
          if (chartInstance) {
            chartInstance.destroy()
          }
          
          const ctx = $$canvas.getContext('2d')
          chartInstance = new chart.Chart(ctx, {
            type: $$type,
            data: {
              datasets: [{
                data: $$data,
                backgroundColor: '#3b82f6'
              }]
            }
          })
        })
        
        onCleanup(() => {
          if (chartInstance) {
            chartInstance.destroy()
          }
        })
      </script>
      
      <canvas data-ref="canvas"></canvas>
    </template>

## Rocket Attributes 

In addition to the Rocket-specific `data-*` attributes defined above, the following attributes are available within Rocket components.

Rocket only transforms Datastar attributes such as `data-text`, `data-on`, and `data-attr`. Custom `data-*` attributes you add for your own semantics (e.g., `data-info="Hello Delaney!"`) are preserved verbatim in the rendered DOM.

By default, Rocket renders into the light DOM of the custom element, so the component’s content participates directly in the page layout and inherits global styles. The shadow attributes `data-shadow-*` let's you opt a component into using a Shadow DOM host instead. If you’re not familiar with Shadow DOM concepts like the [shadow root](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot), it’s worth reading the MDN documentation first.

### Light DOM style scoping 

Light DOM Rocket components automatically scope any `<style>` blocks declared inside the component template and inside the component’s light DOM children. Selectors are rewritten to target only that component instance, so styles won’t leak across instances. Global stylesheets still apply as usual.

Use `:global(...)` in a selector to opt out of scoping for that selector. Shadow DOM components already have native style encapsulation, so scoping is only applied to light DOM components.
    
    
    <template data-rocket:badge-list>
      <style>
        .badge { display: inline-flex; gap: 0.25rem; }
        .badge strong { color: #0a0; }
        :global(.accent) { color: #e11d48; }
      </style>
      <div class="badge">
        <strong data-text="$$label"></strong>
        <slot></slot>
      </div>
    </template>
    
    <badge-list data-attr:label="'Team'">
      <style>
        .badge { background: #fee; border: 1px solid #f99; }
        .badge em { font-style: normal; color: #900; }
      </style>
      <em class="accent">Alpha</em>
    </badge-list>

### `data-shadow-open`

Use `data-shadow-open` to force an **open Shadow DOM** when you want style encapsulation but still need access to internal elements via `element.shadowRoot`, which is useful during debugging or integration.
    
    
    <template data-rocket:tag-pill
              data-shadow-open
              data-props:label="string|trim|required!">
      <style>
        .pill {
          display: inline-flex;
          align-items: center;
          padding: 0.25rem 0.5rem;
          border-radius: 999px;
          background: #0f172a;
          color: white;
          font-size: 0.75rem;
          gap: 0.25rem;
        }
        .dot {
          width: 6px;
          height: 6px;
          border-radius: 999px;
          background: #22c55e;
        }
      </style>
      <div class="pill">
        <span class="dot"></span>
        <span data-text="$$label"></span>
      </div>
    </template>
    
    <!-- Styles are fully encapsulated, but devtools and test harnesses can still inspect the .pill element via element.shadowRoot -->
    <tag-pill data-attr:label="'Shadow-ready'"></tag-pill>

### `data-shadow-closed`

Use `data-shadow-closed` to force a **closed Shadow DOM**. Choose this when you want the implementation to be fully encapsulated and inaccessible via `element.shadowRoot`, while still benefitting from Shadow DOM styling and slot projection.
    
    
    <template data-rocket:status-tooltip
              data-shadow-closed
              data-props:text="string|trim|required!">
      <script>
        $$show = false
      </script>
    
      <span data-on:mouseenter="$$show = true"
            data-on:mouseleave="$$show = false">
        <slot></slot>
        <span data-show="$$show" class="tooltip"
              data-text="$$text"></span>
      </span>
    </template>
    
    <!-- The tooltip DOM is hidden inside a closed shadow root -->
    <status-tooltip data-attr:text="'Hello from Rocket'">
      Hover me
    </status-tooltip>

### `data-if`

Conditionally outputs an element based on an expression. Must be placed on a `<template>` element in Rocket components.
    
    
    <template data-if="$$items.count">
      <div data-text="$$items.count + ' items'"></div>
    </template>

### `data-else-if`

Conditionally outputs an element based on an expression, if the preceding `data-if` condition is falsy. Must be on a `<template>`.
    
    
    <template data-if="$$items.count">
      <div data-text="$$items.count + ' items found.'"></div>
    </template>
    <template data-else-if="$$items.count == 1">
      <div data-text="$$items.count + ' item found.'"></div>
    </template>

### `data-else`

Outputs an element if the preceding `data-if` and `data-else-if` conditions are falsy. Must be on a `<template>`.
    
    
    <template data-if="$$items.count">
      <div data-text="$$items.count + ' items found.'"></div>
    </template>
    <template data-else>
      <div>No items found.</div>
    </template>

### `data-for`

Loops over any iterable (arrays, maps, sets, strings, and plain objects), and outputs the element for each item. Must be placed on a `<template>`.
    
    
    <template data-for="item, index in $$items">
      <div>
        <span data-text="index + ': ' + item.name"></span>
      </div>
    </template>

### `data-key`

Provides a stable key for each iteration when used alongside `data-for`. Keys enable DOM reuse (Solid-like keyed loops) and must live on the same `<template data-for>`.
    
    
    <template data-for="item in $$items" data-key="item.id">
      <div data-text="item.label"></div>
    </template>

The first alias (`item` above) is available to descendants just like any other binding. An optional second alias (`index` above) exposes the current key or numeric index. Nested loops are supported, and inner loop variables automatically shadow outer ones, so you can reuse names without conflicts.
    
    
    <template data-for="items in $$itemSet">
      <div>
        <template data-for="item in items">
          <div>
            <span data-text="item.name"></span>
          </div>
        </template>
      </div>
    </template>

## Reactive Patterns 

Rocket provides `computed` and `effect` functions for declarative reactivity. These keep your component state automatically in sync with the DOM.

### Computed Values 

Computed values automatically update when their dependencies change.
    
    
    <template data-rocket:shopping-cart
              data-props:items="json|=[]"
    >
      <script>
        // Computed values automatically recalculate
        $$total = computed(() => 
          $$items.reduce((sum, item) => sum + (item.price * item.quantity), 0)
        )
        
        $$itemCount = computed(() =>
          $$items.reduce((sum, item) => sum + item.quantity, 0)
        )
        
        $$isEmpty = computed(() => $$items.length === 0)
        
        // Actions that modify reactive state
        action({
          name: 'addItem',
          apply(_, item) {
            $$items = [...$$items, { ...item, quantity: 1 }]
          },
        })
        
        action({
          name: 'removeItem',
          apply(_, index) {
            $$items = $$items.filter((_, i) => i !== index)
          },
        })
      </script>
      
      <div>
        <h3>Shopping Cart</h3>
        <p data-show="$$isEmpty">Cart is empty</p>
        <p data-show="!$$isEmpty">
          Items: <span data-text="$$itemCount"></span> | 
          Total: $<span data-text="$$total.toFixed(2)"></span>
        </p>
        
        <template data-for="item, index in $$items">
          <div>
            <span data-text="item.name"></span> - 
            <span data-text="'$' + item.price"></span>
            <button data-on:click="@removeItem(index)">Remove</button>
          </div>
        </template>
      </div>
    </template>

### Effects and Watchers 

Effects run side effects when reactive values change.
    
    
    <template data-rocket:auto-saver
              data-props:data="string|="
              data-props:last-saved="string|="
              data-props:saving="boolean|=false"
    >
      <script>
        let saveTimeout
        
        // Auto-save effect
        effect(() => {
          if (!$$data) {
            return
          }
          
          clearTimeout(saveTimeout)
          saveTimeout = setTimeout(async () => {
            $$saving = true
            try {
              await actions.post('/api/save', { data: $$data })
              $$lastSaved = new Date().toLocaleTimeString()
            } catch (error) {
              console.error('Save failed:', error)
            } finally {
              $$saving = false
            }
          }, 1000) // Debounce by 1 second
        })
        
        // Theme effect
        effect(() => {
          if ($theme) {
            document.body.className = $theme + '-theme'
          }
        })
        
        onCleanup(() => {
          clearTimeout(saveTimeout)
        })
      </script>
      
      <div>
        <textarea data-bind="data" placeholder="Start typing..."></textarea>
        <p data-show="$$saving">Saving...</p>
        <p data-show="$$lastSaved">Last saved: <span data-text="$$lastSaved"></span></p>
      </div>
    </template>

## Element References 

You can use `data-ref` to create references to elements within your component. Element references are available as `$$elementName` signals and automatically updated when the DOM changes.
    
    
    <template data-rocket:canvas-painter
              data-props:color="string|=#000000"
              data-props:brush-size="int|=5"
    >
      <script>
        let ctx
        let isDrawing = false
        
        // Get canvas context when canvas is available
        effect(() => {
          if ($$canvas) {
            ctx = $$canvas.getContext('2d')
            ctx.strokeStyle = $$color
            ctx.lineWidth = $$brushSize
            ctx.lineCap = 'round'
          }
        })
        
        // Update drawing properties
        effect(() => {
          if (ctx) {
            ctx.strokeStyle = $$color
            ctx.lineWidth = $$brushSize
          }
        })
        
        action({
          name: 'startDrawing',
          apply(_, e) {
            isDrawing = true
            const rect = $$canvas.getBoundingClientRect()
            ctx.beginPath()
            ctx.moveTo(e.clientX - rect.left, e.clientY - rect.top)
          },
        })
        
        action({
          name: 'draw',
          apply(_, e) {
            if (!isDrawing) {
              return
            }
    
            const rect = $$canvas.getBoundingClientRect()
            ctx.lineTo(e.clientX - rect.left, e.clientY - rect.top)
            ctx.stroke()
          },
        })
        
        action({
          name: 'stopDrawing',
          apply() {
            isDrawing = false
          },
        })
        
        action({
          name: 'clear',
          apply() {
            if (ctx) {
              ctx.clearRect(0, 0, $$canvas.width, $$canvas.height)
            }
          },
        })
      </script>
      
      <div>
        <div>
          <label>Color: <input type="color" data-bind="color"></label>
          <label>Size: <input type="range" min="1" max="20" data-bind="brushSize"></label>
          <button data-on:click="@clear()">Clear</button>
        </div>
        
        <canvas 
          data-ref="canvas" 
          width="400" 
          height="300"
          style="border: 1px solid #ccc"
          data-on:mousedown="@startDrawing"
          data-on:mousemove="@draw"
          data-on:mouseup="@stopDrawing"
          data-on:mouseleave="@stopDrawing">
        </canvas>
      </div>
    </template>

## Validation with Codecs 

Rocket’s built-in codec system makes it possible to validate user input. By defining validation rules directly in your `data-props:*` attributes, data is automatically transformed and validated as it flows through your component.

### Type Codecs 

Type codecs convert and validate prop values.
    
    
    <template data-rocket:validated-form
              data-props:email="string|trim|required!|="
              data-props:age="int|min:18|max:120|=0"
              data-props:score="int|clamp:0,100|=0"
    >
      <script>
        // Signals are automatically validated by the codec system
        // No need for manual codec setup - just use the signals directly
        
        // Check for validation errors using the built-in $$hasErrs signal
        // No need to create computed - $$hasErrs is automatically available
      </script>
      
      <form>
        <div>
          <label>Email (required):</label>
          <input type="email" data-bind="email">
          <span data-show="$$errs?.email" class="error">Email is required</span>
        </div>
        
        <div>
          <label>Age (18-120):</label>
          <input type="number" data-bind="age">
          <span data-show="$$errs?.age" class="error">Age must be 18-120</span>
        </div>
        
        <div>
          <label>Score (0-100, auto-clamped):</label>
          <input type="number" data-bind="score">
          <span>Current: <span data-text="$$score"></span></span>
        </div>
        
        <button type="submit" data-attr:disabled="$$hasErrors">
          Submit
        </button>
      </form>
    </template>

For date props, omitting an explicit default will use the current time. This is evaluated when the codec runs, producing a fresh `Date` instance based on the current time.
    
    
    <template data-rocket:last-updated
              data-props:serverUpdateTime="date"
    >
                <script>
        $$formatted = computed(() => $$serverUpdateTime.toLocaleString())
            </script>
      
            <span data-text="$$formatted"></span>
    </template>

### Validation Rules 

Codecs can either **transform** values (modify them) or **validate** them (check them without modifying). Use the `!` suffix to make any codec validation-only.

  * `min:10` \- Transform: clamps value to minimum 10
  * `min:10!` \- Validate: rejects values below 10, keeps original on failure
  * `trim` \- Transform: removes whitespace
  * `trim!` \- Validate: rejects untrimmed strings

Codec| Transform| Validation  
---|---|---  
**Type Conversion**  
`string`| Converts to string| Is string?  
`int`| Converts to integer| Is integer?  
`float`| Converts to number| Is numeric?  
`date`| Converts ISO strings or timestamps to a `Date` object (defaults to the current time)| Is valid date?  
`boolean`| Converts to boolean. A missing attribute decodes to `false` by default, while a present-but-empty attribute (e.g. `<foo-bar baz>` on a `baz` prop) decodes to `true`.| Is boolean?  
`json`| Parses JSON string| Valid JSON?  
`js`| Parses JS object literal  
**⚠️[Avoid client values](https://xkcd.com/327/)**| Valid JS syntax?  
`binary`| Decodes base64| Valid base64?  
**Validation**  
`required`| -| Not empty?  
`oneOf:a,b,c`| Defaults to first option if invalid| Is valid option?  
**Numeric Constraints**  
`min:n`| Clamp to minimum value| >= minimum?  
`max:n`| Clamp to maximum value| <= maximum?  
`clamp:min,max`| Clamp between min and max| In range?  
`round` / `round:n`| Round to n decimal places| Is rounded?  
`ceil:n` / `floor:n`| Ceiling/floor to n decimal places| Is ceiling/floor?  
**String Transforms**  
`trim`| Remove leading/trailing whitespace| -  
`upper` / `lower`| Convert to upper/lowercase| -  
`kebab` / `camel`| Convert case style| Correct case?  
`snake` / `pascal`| Convert case style| Correct case?  
`title` / `title:first`| Title case (all words or first only)| -  
**String Constraints**  
`minLength:n`| -| Length >= n?  
`maxLength:n`| Truncates if too long| Length <= n?  
`length:n`| -| Length equals n?  
`regex:pattern`| -| Matches regex?  
`startsWith:text`| Adds prefix if missing| Starts with text?  
`endsWith:text`| Adds suffix if missing| Ends with text?  
`includes:text`| -| Contains text?  
**Advanced Numeric**  
`lerp:min,max`| Linear interpolation (0-1 to min-max)| -  
`fit:in1,in2,out1,out2`| Map value from one range to another| -  
  
## Component Lifecycle 

Rocket components have a simple lifecycle with automatic cleanup.
    
    
    <template data-rocket:lifecycle-demo>
      <script>
        console.log('Component initializing...')
        
        $$mounted = true
        
        // Setup effects and timers
        const intervalId = setInterval(() => {
          console.log('Component is alive')
        }, 5000)
        
        // Cleanup when component is removed from DOM
        onCleanup(() => {
          console.log('Component cleanup')
          clearInterval(intervalId)
          $$mounted = false
        })
      </script>
      
      <div>
        <p data-show="$$mounted">Component is mounted</p>
      </div>
    </template>

The lifecycle is as follows: 

  1. Rocket processes your template and registers the component.
  2. When you add it to the DOM, the instance is created and setup scripts run to initialize your signals.
  3. The component becomes reactive and responds to data changes.
  4. When you remove it from the DOM, all `onCleanup` callbacks run automatically.

## Optimistic UI 

Rocket pairs seamlessly with Datastar’s server-driven model to provide instant visual feedback without shifting ownership of state to the browser. In the [Rocket flow example](/examples/rocket_flow), dragging a node instantly renders its optimistic position in the SVG while the original light-DOM host remains hidden. The component adds an `.is-pending` class to dim the node and connected edges, signaling that the drag is provisional. Once the backend confirms the new coordinates and updates the layout, the component automatically clears the pending style.

A dedicated prop such as `server-update-time="date"` makes this straightforward: each tab receives an updated timestamp from the server (via SSE or a patch), Rocket decodes it into a `Date` (defaulting to the current time when no value is provided), and internal effects react to reconcile every view. Unlike client-owned graph editors (e.g. React Flow), the server stays the single source of truth, while the optimistic UI remains a thin layer inside the component.

## Examples 

Check out the [Copy Button](/examples/rocket_copy_button) as a basic example, the [QR Code generator](/examples/rocket_qr_code) with validation, the [ECharts integration](/examples/rocket_echarts) for data visualization, the interactive [3D Globe](/examples/rocket_globe) with markers, and the [Virtual Scroll](/examples/rocket_virtual_scroll) example for handling large datasets efficiently.
