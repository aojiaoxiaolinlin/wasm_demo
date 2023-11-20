import { ref } from "vue";

export default function DemoHooks() {
  let num = ref(1000);

  function addX(x: number) {
    num.value += x;
  }
  function subtraction(x: number) {
    num.value -= x;
  }
  return {
    num,
    addX,
    subtraction,
  };
}
