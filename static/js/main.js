import './vendor/vue.js'
import { api, buildFileTreeRecursively } from './util.js'

import Directory from './components/directory.js'



const app = new Vue({
  components: {
    Directory,
  },
  computed: {
    _dirList() {
      return Object.values(this.dirs)
    },
    rootDir() {
      return buildFileTreeRecursively(
        this.dirs,
        this._dirList.find(dir => dir.name === 'root') || {}
      )
    },
  },
  data() {
    return {
      newFileName: '',
      dirs: {},
      dirsOpenState: {},
      errors: []
    }
  },
  methods: {
    async init() {
      try {
        this.dirs = {}
        this.dirsOpenState = {}
        const { data } = await api.doGet('/dir/list')
        data.forEach(dir => this.fetchDir(dir.id))
      } catch (error) {
        this.errors.push(error)
      }
    },
    async fetchDir(dirId) {
      try {
        const { data } = await api.doGet(`/dir/${dirId}`)
        this.$set(this.dirs, dirId, data)
        this.$set(this.dirsOpenState, dirId, data.name === 'root' || !!this.dirsOpenState[dirId])
      } catch (error) {
        this.errors.push(error)
      }
    },
    async onCreateNewFile(parentDirId) {
      try {
        await api.doPost('/file/', {
          name: this.newFileName,
          parent_dir_id: parentDirId
        })
        this.init()
      } catch (error) {
        console.error(error)
        this.errors.push(error)
      }
    },
    onToggleOpenState({ id, isOpen }) {
      this.dirsOpenState[id] = isOpen
    }
  },
  mounted() {
    this.init()
  },
  template: `
    <div id="app">
      <div class="page">
        <div class="page__file-tree">
          <div>
            <form @submit.prevent="onCreateNewFile(rootDir.id)">
              <div class="input">
                <label class="input__label">New File</label>
                <input class="input__input" type="text" v-model="newFileName" />
              </div>
              <button type="submit">Submit</button>
            </form>
          </div>
          <div>
            <div class="dir-tree" v-if="rootDir.id">
              <Directory
                :dirsOpenState="dirsOpenState"
                @toggleOpenState="onToggleOpenState"
                v-bind="rootDir"
              />
            </div>
          </div>
        </div>
        <div class="page__main-content">
          main content
        </div>
      </div>
    </div>
  `,
})

app.$mount('#app')


