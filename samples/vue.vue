<template>
  <div id="app">
    <!-- Template Syntax -->
    <h1>{{ title }}</h1>
    <p>{{ message }}</p>
    
    <!-- Directives -->
    <div v-if="isVisible">This is conditionally rendered</div>
    <div v-else>Alternative content</div>
    
    <p v-show="showElement">This element's visibility is toggled</p>
    
    <!-- List Rendering -->
    <ul>
      <li v-for="(item, index) in items" :key="item.id">
        {{ index + 1 }}. {{ item.name }} - {{ item.category }}
      </li>
    </ul>
    
    <!-- Event Handling -->
    <button @click="increment">Count: {{ count }}</button>
    <button @click="decrement" :disabled="count <= 0">Decrement</button>
    
    <!-- Two-way Data Binding -->
    <div>
      <input v-model="inputValue" placeholder="Type something...">
      <p>You typed: {{ inputValue }}</p>
    </div>
    
    <!-- Form Handling -->
    <form @submit.prevent="handleSubmit">
      <div>
        <label>Name:</label>
        <input v-model="form.name" type="text" required>
      </div>
      <div>
        <label>Email:</label>
        <input v-model="form.email" type="email" required>
      </div>
      <div>
        <label>
          <input v-model="form.subscribe" type="checkbox">
          Subscribe to newsletter
        </label>
      </div>
      <div>
        <label>Gender:</label>
        <input v-model="form.gender" type="radio" value="male" id="male">
        <label for="male">Male</label>
        <input v-model="form.gender" type="radio" value="female" id="female">
        <label for="female">Female</label>
      </div>
      <div>
        <label>Country:</label>
        <select v-model="form.country">
          <option value="">Select a country</option>
          <option value="us">United States</option>
          <option value="ca">Canada</option>
          <option value="uk">United Kingdom</option>
        </select>
      </div>
      <button type="submit">Submit</button>
    </form>
    
    <!-- Class and Style Bindings -->
    <div :class="{ active: isActive, 'text-danger': hasError }">
      Dynamic classes
    </div>
    <div :style="{ color: textColor, fontSize: fontSize + 'px' }">
      Dynamic styles
    </div>
    
    <!-- Component Usage -->
    <UserCard 
      v-for="user in users" 
      :key="user.id"
      :user="user"
      @user-clicked="handleUserClick"
    />
    
    <!-- Slots -->
    <Modal v-if="showModal" @close="showModal = false">
      <template #header>
        <h3>Modal Title</h3>
      </template>
      <template #default>
        <p>This is the modal content.</p>
      </template>
      <template #footer>
        <button @click="showModal = false">Close</button>
      </template>
    </Modal>
    
    <!-- Computed Properties Demo -->
    <div>
      <p>Filtered items ({{ filteredItems.length }}):</p>
      <ul>
        <li v-for="item in filteredItems" :key="item.id">
          {{ item.name }}
        </li>
      </ul>
      <input v-model="filter" placeholder="Filter items...">
    </div>
    
    <!-- Watchers Demo -->
    <div>
      <input v-model="watchedValue" placeholder="Type to trigger watcher">
      <p v-if="watchedMessage">{{ watchedMessage }}</p>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, watch, onMounted } from 'vue'
import UserCard from './components/UserCard.vue'
import Modal from './components/Modal.vue'

export default {
  name: 'App',
  components: {
    UserCard,
    Modal
  },
  setup() {
    // Reactive data
    const title = ref('Vue.js Comprehensive Example')
    const message = ref('Welcome to Vue.js!')
    const isVisible = ref(true)
    const showElement = ref(true)
    const count = ref(0)
    const inputValue = ref('')
    const filter = ref('')
    const watchedValue = ref('')
    const watchedMessage = ref('')
    const showModal = ref(false)
    const isActive = ref(true)
    const hasError = ref(false)
    const textColor = ref('#333')
    const fontSize = ref(16)
    
    // Reactive objects
    const form = reactive({
      name: '',
      email: '',
      subscribe: false,
      gender: '',
      country: ''
    })
    
    const items = ref([
      { id: 1, name: 'Apple', category: 'Fruit' },
      { id: 2, name: 'Carrot', category: 'Vegetable' },
      { id: 3, name: 'Banana', category: 'Fruit' },
      { id: 4, name: 'Broccoli', category: 'Vegetable' }
    ])
    
    const users = ref([
      { id: 1, name: 'John Doe', email: 'john@example.com' },
      { id: 2, name: 'Jane Smith', email: 'jane@example.com' }
    ])
    
    // Computed properties
    const filteredItems = computed(() => {
      if (!filter.value) return items.value
      return items.value.filter(item => 
        item.name.toLowerCase().includes(filter.value.toLowerCase())
      )
    })
    
    // Methods
    const increment = () => {
      count.value++
    }
    
    const decrement = () => {
      if (count.value > 0) {
        count.value--
      }
    }
    
    const handleSubmit = () => {
      console.log('Form submitted:', form)
      alert('Form submitted! Check console for details.')
    }
    
    const handleUserClick = (user) => {
      alert(`User clicked: ${user.name}`)
    }
    
    // Watchers
    watch(watchedValue, (newVal, oldVal) => {
      if (newVal) {
        watchedMessage.value = `Value changed from "${oldVal}" to "${newVal}"`
      } else {
        watchedMessage.value = ''
      }
    })
    
    // Lifecycle hooks
    onMounted(() => {
      console.log('Component mounted!')
    })
    
    return {
      title,
      message,
      isVisible,
      showElement,
      count,
      inputValue,
      filter,
      watchedValue,
      watchedMessage,
      showModal,
      isActive,
      hasError,
      textColor,
      fontSize,
      form,
      items,
      users,
      filteredItems,
      increment,
      decrement,
      handleSubmit,
      handleUserClick
    }
  }
}
</script>

<style scoped>
#app {
  font-family: Arial, sans-serif;
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

h1 {
  color: #42b883;
  text-align: center;
}

button {
  background-color: #42b883;
  color: white;
  border: none;
  padding: 8px 16px;
  margin: 4px;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #369870;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

input, select {
  padding: 8px;
  margin: 4px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

form {
  background-color: #f9f9f9;
  padding: 20px;
  border-radius: 8px;
  margin: 20px 0;
}

form div {
  margin-bottom: 10px;
}

label {
  display: inline-block;
  width: 100px;
  font-weight: bold;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  background-color: #f0f0f0;
  margin: 4px 0;
  padding: 8px;
  border-radius: 4px;
}

.active {
  font-weight: bold;
}

.text-danger {
  color: red;
}

/* Responsive design */
@media (max-width: 600px) {
  #app {
    padding: 10px;
  }
  
  label {
    width: auto;
    display: block;
    margin-bottom: 4px;
  }
}
</style>