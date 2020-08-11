import './vendor/vue.js'
import { api } from './util.js'


const app = new Vue({
  computed: {
    rootDir() {
      return this.directories.find(d => d.name === 'root')
    }
  },
  data() {
    return {
      counter: 1,
      dirList: [],
      dirContentList: [],
      errors: []
    }
  },
  methods: {

  },
  async mounted() {
    try {
      this.dirList = await api.doGet('/api/dir/list')
      this.dirContentList = await Promise.all(
        this.dirList.map(dir => api.doGet(`/api/dir/${dir.id}`))
      )
    } catch (error) {
      this.errors.push(error.message)
    }
  },
  template: `
    <div>
      Hello from the app {{counter}} {{counter < 2 ? 'time' : 'times'}}
      <button @click="counter += 1">click me</button>
    </div>
  `,
})

app.$mount('#app')


