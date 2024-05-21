const client_api_url: string = "http://127.0.0.1:8089/api/menu";

export function setupSubmit(buttonElement: HTMLButtonElement) {
  let sendOptions = (): void => {
    let request_obj = new AssembleRequest();
    request_obj.send();
  };

  buttonElement.addEventListener("click", () => sendOptions());
}

class AssembleRequest {
  prediction: boolean;
  treasure: boolean;
  xMaps: boolean;
  riddles: boolean;
  fovMultiplier: number;
  maxSimNum: number;
  movementMode: number;
  instantLadder: boolean;
  extendedReach: boolean;
  increaseSpeed: boolean;
  forceMovement: boolean;

  constructor() {
    this.prediction = $("#drawPred").is(":checked");
    this.treasure = $("#drawTreasure").is(":checked");
    this.xMaps = $("#drawXMaps").is(":checked");
    this.riddles = $("#drawRiddles").is(":checked");

    this.fovMultiplier = parseFloat($("#fovMultiplier").val());
    this.maxSimNum = parseInt($("#maxSimulationNum").val());
    this.movementMode = parseInt($("#movementMode").val());

    this.forceMovement = $("#forceMovement").is(":checked");
    this.instantLadder = $("#instantLadder").is(":checked");
    this.extendedReach = $("#extendedReach").is(":checked");
    this.increaseSpeed = $("#increaseSpeed").is(":checked");
  }

  async send() {
    try {
      const jsonData = JSON.stringify(this);

      const requestOptions: RequestInit = {
        method: "POST",
        mode: "no-cors",
        headers: {
          "Content-Type": "application/json",
          Accept: "*/*",
        },
        body: jsonData,
      };

      const response = await fetch(client_api_url, requestOptions);

      $("#requestStatus").empty();
      $("#requestStatus").append("Sucessful update");
    } catch (error) {
      $("#requestStatus").empty();
      $("#requestStatus").append("Error updating..." + error);
    }
  }
}
