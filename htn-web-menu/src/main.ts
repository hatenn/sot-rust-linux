import { setupSubmit } from "./menu";
import "./style.css";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1>Menu</h1>
    <div class="card">
      <form id="menu-form" onsubmit="return false">
      <h2>ESP</h2>
      <div class="form-check">
        <label class="custom-checkbox" for="drawPred">
          Prediction
          <input type="checkbox" id="drawPred"/>
          <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
          <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
        </label>
      </div>
      <div class="form-check">
        <label class="custom-checkbox" for="drawTreasure">
          Treasure
          <input type="checkbox" id="drawTreasure"/>
          <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
          <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
        </label>
      </div>
      <div class="form-check">
        <label class="custom-checkbox" for="drawXMaps">
          XMaps
          <input type="checkbox" id="drawXMaps"/>
          <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
          <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
        </label>
      </div>
      <div class="form-check">
        <label class="custom-checkbox" for="drawRiddles">
          Riddles
          <input type="checkbox" id="drawRiddles"/>
          <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
          <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
        </label>
      </div>
      <h2>Misc</h2>
      <div>
        <label for="maxSimulationNum">FOV Multiplier</label>
        <p>Value: <output id="value"></output></p>
        <input type="range" class="form-control" id="fovMultiplier" min="1.00" max="1.30" step="0.02">
      </div>
        <div>
          <label for="maxSimulationNum">Max Simulation Number</label>
          <input type="number" class="form-control" id="maxSimulationNum" placeholder="8">
        </div>
        <div>
          <label for="movementMode">Movement Mode</label>
          <input type="number" class="form-control" id="movementMode" placeholder="4">
        </div>
        <div class="form-check">
          <label class="custom-checkbox" for="forceMovement">
            Force Movement Mode
            <input type="checkbox" id="forceMovement"/>
            <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
            <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
          </label>
        </div>
        <div class="form-check">
          <label class="custom-checkbox" for="instantLadder">
            Instant Ladder
            <input type="checkbox" id="instantLadder"/>
            <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
            <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
          </label>
        </div>
        <div class="form-check">
          <label class="custom-checkbox" for="extendedReach">
            Extended Reach
            <input type="checkbox" id="extendedReach"/>
            <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
            <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
          </label>
        </div>
        <div class="form-check">
          <label class="custom-checkbox" for="increaseSpeed">
            Increase Speed
            <input type="checkbox" id="increaseSpeed"/>
            <img src="/unchecked.svg" class="skull-check unchecked" alt="UncheckedMark">
            <img src="/checked.svg" class="skull-check checked" alt="UncheckedMark">
          </label>
        </div>
        <p id="requestStatus"></p>
        <button type="submit" class="btn btn-primary" id="submitOptions">Apply</button>
      </form>
      <h2>Internal Functions</h2>
      <button type="submit" class="btn btn-primary" id="tpShip">Teleport to Ship</button>
      <button type="submit" class="btn btn-primary" id="reanimate">Reanimate</button>
    </div>
    <p class="read-the-docs">
      Each man shall keep his piece, cutlass, and pistols at all times clean and ready for action.
    </p>
  </div>
`;

const value = document.querySelector("#value");
const input = document.querySelector("#fovMultiplier");
value.textContent = input.value;
input.addEventListener("input", (event) => {
  value.textContent = event.target.value;
});

setupSubmit(document.querySelector<HTMLButtonElement>("#submitOptions")!);
