<script setup lang="ts">
import animation from '../assets/jsons/spirit2159src.json';
import { onMounted, ref } from 'vue'
import init, { add, greet, aes_gcm_encrypt } from 'hello-wasm'

defineProps<{ msg: string }>()

const count = ref(0)
onMounted(() => {
  init().then(() => {
    console.log('Wasm module loaded');
    add(1, 2);
    const str = JSON.stringify(animation);
    const start = Date.now();
    const res = aes_gcm_encrypt(str);
    console.log('aes_gcm_encrypt', Date.now() - start);
    console.log(res.get_key());
    console.log(res.get_encrypt_data());
  })
})
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <button type="button" @click="greet('混合')">count is {{ count }}</button>
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
    Learn more about IDE Support for Vue in the
    <a href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support" target="_blank">Vue Docs Scaling up Guide</a>.
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
