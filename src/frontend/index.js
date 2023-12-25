// FIXME: don't import from the web
import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);

function App(props) {
  return html`
    <div>
      ${props.files.map((file) => {
        return html`
        <div class="file">
          <label>${file}</label>
        </div>`;
      })}
    </div>
  `;
}

let i = 0;

let update = async () => {
  let response = await fetch("/api/files/info");
  if (response.status !== 200) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
 
  let json = await response.json();
  render(html`<${App} files=${json}></${App}>`, document.body);
};

update();
setInterval(update, 200);
