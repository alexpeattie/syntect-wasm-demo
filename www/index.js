import { render } from '../pkg'
import debounce from 'debounce'

function highlightCode(input) {
  const syntectOutput = render(input.value)
  document.querySelector('.highlighted').innerHTML = syntectOutput
}
const highlightCodeDebounced = debounce(highlightCode, 250)

const placeholder = `const main = document.querySelector("main");
for (const link of document.querySelectorAll("nav > a")) {
  link.addEventListener("click", e => {
    e.preventDefault();

    import('/modules/my-module.js')
      .then(module => {
        module.loadPageInto(main);
      })
      .catch(err => {
        main.textContent = err.message;
      });
  });
}`

const code = document.querySelector('#code')
code.value = placeholder
code.addEventListener('keyup', (e) => highlightCodeDebounced(e.target))
code.addEventListener('change', (e) => highlightCode(e.target))
highlightCode(code)
