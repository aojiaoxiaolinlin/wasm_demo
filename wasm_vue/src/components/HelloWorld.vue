<script setup lang="ts">
import { ref, onMounted } from 'vue'
import init, { greet,add } from 'hello-wasm'
import DemoHooks from '../hooks/demo';
defineProps<{ msg: string }>()
onMounted(() => {
  init().then((instance) => {
    console.log(instance);
    greet("wasm--霖霖")
  })
})
const count = ref(0)
const onclick = () => {
  add(count.value,++count.value)
}

const {num,addX,subtraction} = DemoHooks();

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="onclick">count is {{ count }}</button>
    <br>
    <button @click="addX(1)">num is {{ num }}</button>
    <button @click="subtraction(num)">num is {{ num }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank">create-vue</a>, the official Vue + Vite
    starter
  </p>
  <p>
    Install
    <a href="https://github.com/vuejs/language-tools" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
