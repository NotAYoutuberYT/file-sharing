// FIXME: don't import from the web
import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

// TODO: find a smarter method of updating (webhooks?)
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
setInterval(update, 1000);

document.getElementById('uploadForm').addEventListener('submit', function(event) {
  event.preventDefault();
  var fileInput = document.getElementById('file');

  if (fileInput.files.length > 0) {
      var file = fileInput.files[0];

      var formData = new FormData();
      formData.append(file.name, file);

      fetch('/api/upload', {
          method: 'POST',
          body: formData
      })
  }
});
