<script setup lang="ts">
import { ref, onMounted } from 'vue'
import init, { Universe } from 'hello-wasm'
const gameOfLifeCanvas = ref<any>()

onMounted(() => {
  init().then((instance) => {
    console.log(instance);
    const universe = Universe.new()

    const renderLoop = () => {
      if (gameOfLifeCanvas.value) {        
        gameOfLifeCanvas.value.textContent = universe.render()
        universe.tick()
      }
      requestAnimationFrame(renderLoop)

    }
    requestAnimationFrame(renderLoop)
  })
})
</script>

<template>
  <div>
    <pre ref="gameOfLifeCanvas" ></pre>
  </div>
</template>

<style scoped>
div {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
</style>
