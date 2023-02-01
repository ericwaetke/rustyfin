import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const [password, setPassword] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name(), password: password() }));
  }

  const [databaseString, setDatabaseString] = createSignal("");
  async function getDatabaseItems () {
    invoke("get_saved_user").then((res) => {
      console.log(res)
     
    }
    ).catch((err) => {
      console.log(err)
      });
  }

  return (
    <div class="container">
      <h1>Welcome to Rustyfin!</h1>

      <div class="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <input
            id="password-input"
            onChange={(e) => setPassword(e.currentTarget.value)}
            type="password"
            placeholder="Enter a password..."
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
      </div>

      

      <p>{greetMsg}</p>


      <div class="row">
        <div>
          <button type="button" onClick={() => getDatabaseItems()}>
            Get Database Items
          </button>
        </div>
      </div>

      <p>
        {databaseString}
      </p>
    </div>
  );
}

export default App;
