import './vendor/vue.js'
import { api } from './util.js'


const app = new Vue({
  computed: {
    rootDir() {
      return this.dirList.find(dir => dir.name === 'root') || {}
    },
    dirIds() {
      return Object.keys(this.dirs)
    },
    dirList() {
      return Object.values(this.dirs)
    },
    dirTree() {

      const visitedDirs = {}
      
      const buildTreeRecursive = (accum, dir) => {
        accum[dir.id] = dir

        dir.child_dirs.forEach(childDir => {

          accum[dir.id].child_dirs = buildTreeRecursive(accum, this.dirs[childDir.id])

        })
      
        return accum
      }

      if (this.rootDir) {
        return buildTreeRecursive({}, this.rootDir)
      }
      return {}
    }
  },
  data() {
    return {
      newFileName: '',
      dirs: {},
      errors: []
    }
  },
  methods: {
    async init() {
      try {
        this.dirs = {}
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
            <ul>
              <li v-for="(file, i) in rootDir.files">
                {{file.name}}
              </li>
            </ul>
          </div>
        </div>
        <div class="page__main-content">
          main content
          <pre>
            {{rootDir}}
          </pre>
        </div>
      </div>
    </div>
  `,
})

app.$mount('#app')


