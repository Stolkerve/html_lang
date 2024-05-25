    require.config({paths: {'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.49.0/min/vs'}});
    require(['vs/editor/editor.main'], function () {
      monaco.editor.defineTheme('default', {
        "base": "vs-dark",
        "inherit": true,
        "rules": [
          {
            "background": "272822",
            "token": ""
          },
          {
            "foreground": "75715e",
            "token": "comment"
          },
          {
            "foreground": "e6db74",
            "token": "string"
          },
          {
            "foreground": "ae81ff",
            "token": "constant.numeric"
          },
          {
            "foreground": "ae81ff",
            "token": "constant.language"
          },
          {
            "foreground": "ae81ff",
            "token": "constant.character"
          },
          {
            "foreground": "ae81ff",
            "token": "constant.other"
          },
          {
            "foreground": "f92672",
            "token": "keyword"
          },
          {
            "foreground": "f92672",
            "token": "storage"
          },
          {
            "foreground": "66d9ef",
            "fontStyle": "italic",
            "token": "storage.type"
          },
          {
            "foreground": "a6e22e",
            "fontStyle": "underline",
            "token": "entity.name.class"
          },
          {
            "foreground": "a6e22e",
            "fontStyle": "italic underline",
            "token": "entity.other.inherited-class"
          },
          {
            "foreground": "a6e22e",
            "token": "entity.name.function"
          },
          {
            "foreground": "fd971f",
            "fontStyle": "italic",
            "token": "variable.parameter"
          },
          {
            "foreground": "f92672",
            "token": "entity.name.tag"
          },
          {
            "foreground": "a6e22e",
            "token": "entity.other.attribute-name"
          },
          {
            "foreground": "66d9ef",
            "token": "support.function"
          },
          {
            "foreground": "66d9ef",
            "token": "support.constant"
          },
          {
            "foreground": "66d9ef",
            "fontStyle": "italic",
            "token": "support.type"
          },
          {
            "foreground": "66d9ef",
            "fontStyle": "italic",
            "token": "support.class"
          },
          {
            "foreground": "f8f8f0",
            "background": "f92672",
            "token": "invalid"
          },
          {
            "foreground": "f8f8f0",
            "background": "ae81ff",
            "token": "invalid.deprecated"
          },
          {
            "foreground": "cfcfc2",
            "token": "meta.structure.dictionary.json string.quoted.double.json"
          },
          {
            "foreground": "75715e",
            "token": "meta.diff"
          },
          {
            "foreground": "75715e",
            "token": "meta.diff.header"
          },
          {
            "foreground": "f92672",
            "token": "markup.deleted"
          },
          {
            "foreground": "a6e22e",
            "token": "markup.inserted"
          },
          {
            "foreground": "e6db74",
            "token": "markup.changed"
          },
          {
            "foreground": "ae81ffa0",
            "token": "constant.numeric.line-number.find-in-files - match"
          },
          {
            "foreground": "e6db74",
            "token": "entity.name.filename.find-in-files"
          }
        ],
        "colors": {
          "editor.foreground": "#F8F8F2",
          "editor.background": "#272822",
          "editor.selectionBackground": "#49483E",
          "editor.lineHighlightBackground": "#3E3D32",
          "editorCursor.foreground": "#F8F8F0",
          "editorWhitespace.foreground": "#3B3A32",
          "editorIndentGuide.activeBackground": "#9D550FB0",
          "editor.selectionHighlightBorder": "#222218"
        }
      });
      monaco.editor.setTheme('default')
      var editor = monaco.editor.create(document.getElementById('container'), {
        value: `<main>
    <data value="7"></data>      <!-- push n number of fibonacci --> 
    <a href="javascript:fib()"></a>
    
    <output></output>  <!-- print the fibonacci output -->
</main>

<dfn id="fib">
    <var title="num"></var>            <!-- save the arg in a variable num -->

    <cite>num</cite>                   <!-- load num -->
    <data value="1"></data>            <!-- push 1 -->
    <small></small>                    <!-- test if the arg is smaller -->
    <i>                                <!-- if it is -->
        <data value="0"></data>        <!-- push 0 -->
        <rt></rt>                      <!-- return -->
    </i>

    <cite>num</cite>                   <!-- load num -->
    <data value="1"></data>            <!-- push 1 -->
    <em></em>                          <!-- test if equal -->
    <i>                                <!-- if it is -->
        <data value="1"></data>        <!-- push 1 -->
        <rt></rt>                      <!-- return -->
    </i>

    <cite>num</cite>                   <!-- load num -->
    <data value="2"></data>            <!-- push 2 -->
    <em></em>                          <!-- test if equal -->
    <i>                                <!-- if it is -->
        <data value="1"></data>        <!-- push 1 -->
        <rt></rt>                      <!-- return -->
    </i>

    <a href="javascript:fib()">        <!-- recursively invoke fib() on arg - 1 -->
        <cite>num</cite>
        <data value="1"></data>
        <sub></sub>
    </a>

    <a href="javascript:fib()">        <!-- recursively invoke fib() on arg - 2 -->
        <cite>num</cite>
        <data value="2"></data>
        <sub></sub>
    </a>

    <dd></dd>                           <!-- add the results and return -->
</dfn>`,
        language: 'html',
        base: 'vs-dark',
        automaticLayout: true,
      });

      let outputBox = document.getElementById("output")
      let submitBtn = document.getElementById("submitBtn")
      // let outputContainer = document.getElementById("output_container")

      submitBtn.onclick = () => {
        submitBtn.disabled = true;
        submitBtn.textContent = "(?_?)"
        outputBox.textContent = "..."

        fetch("https://html-lang.onrender.com/exec_html_the_programming_language", {
          method: "POST",
          body: editor.getValue()
        }).then((resp) => {
          resp.text().then((text) => {
            outputBox.innerText = text
            submitBtn.disabled = false
            submitBtn.textContent = "Submit (☞⌐▀͡ ͜ʖ͡▀)☞"
          }).catch(() => {
            submitBtn.disabled = false
            submitBtn.textContent = "Submit (☞⌐▀͡ ͜ʖ͡▀)☞"
            outputBox.textContent = ""
          })
        }).catch((resp) => {
          submitBtn.disabled = false
          submitBtn.textContent = "Submit (☞⌐▀͡ ͜ʖ͡▀)☞"
          outputBox.textContent = ""
        })
      }
    })
