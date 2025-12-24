<script setup lang="ts">
import { ref } from 'vue'
import PrintCardView from './PrintCardView.vue'
import type { ControlCard } from '../types/calendar'
import { useCardExport } from '../composables/useCardExport'
import '../styles/print-dialog.scss'

interface Props {
  modelValue: boolean
  card: ControlCard | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const printCardRef = ref<{ $el: HTMLElement } | null>(null)
const {
  exporting,
  printCard,
  exportAsImage,
  exportAsPDF,
  exportAsWord,
  copyToClipboard
} = useCardExport(printCardRef)

const handleClose = () => {
  emit('update:modelValue', false)
}

const handlePrint = async () => {
  // #region agent log
  fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handlePrint',message:'handlePrint called',data:{hasCard:!!props.card,cardId:props.card?.id,printCardRefExists:!!printCardRef.value,printCardRefElExists:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
  // #endregion
  if (props.card) {
    await printCard(props.card)
  } else {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handlePrint',message:'No card in props',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'B'})}).catch(()=>{});
    // #endregion
  }
}

const handleExportImage = async () => {
  // #region agent log
  fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleExportImage',message:'handleExportImage called',data:{hasCard:!!props.card,cardId:props.card?.id,printCardRefExists:!!printCardRef.value,printCardRefElExists:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'A'})}).catch(()=>{});
  // #endregion
  if (props.card) {
    await exportAsImage(props.card)
  } else {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleExportImage',message:'No card in props',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'B'})}).catch(()=>{});
    // #endregion
  }
}

const handleExportPDF = async () => {
  // #region agent log
  fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleExportPDF',message:'handleExportPDF called',data:{hasCard:!!props.card,cardId:props.card?.id,printCardRefExists:!!printCardRef.value,printCardRefElExists:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'A'})}).catch(()=>{});
  // #endregion
  if (props.card) {
    await exportAsPDF(props.card)
  } else {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleExportPDF',message:'No card in props',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'B'})}).catch(()=>{});
    // #endregion
  }
}

const handleExportWord = async () => {
  // #region agent log
  fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleExportWord',message:'handleExportWord called',data:{hasCard:!!props.card,cardId:props.card?.id},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'A'})}).catch(()=>{});
  // #endregion
  if (props.card) {
    await exportAsWord(props.card)
  } else {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleExportWord',message:'No card in props',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'B'})}).catch(()=>{});
    // #endregion
  }
}

const handleCopy = async () => {
  // #region agent log
  fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleCopy',message:'handleCopy called',data:{hasCard:!!props.card,cardId:props.card?.id,printCardRefExists:!!printCardRef.value,printCardRefElExists:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'A'})}).catch(()=>{});
  // #endregion
  if (props.card) {
    await copyToClipboard(props.card)
  } else {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'PrintCardDialog.vue:handleCopy',message:'No card in props',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'run1',hypothesisId:'B'})}).catch(()=>{});
    // #endregion
  }
}
</script>

<template>
  <div v-if="modelValue && card" class="print-dialog-overlay" @click.self="handleClose">
    <div class="print-dialog">
      <div class="print-dialog-header">
        <h2>Печать и экспорт карточки</h2>
        <button class="close-button" @click="handleClose" type="button">×</button>
      </div>
      
      <div class="print-dialog-body">
        <div class="print-preview-container">
          <PrintCardView ref="printCardRef" :card="card" />
        </div>
        
        <div class="print-actions">
          <button
            class="button button-primary"
            @click="handlePrint"
            type="button"
            :disabled="exporting"
          >
            🖨️ Печать
          </button>
          
          <button
            class="button button-secondary"
            @click="handleExportImage"
            type="button"
            :disabled="exporting"
          >
            📷 Сохранить как фото
          </button>
          
          <button
            class="button button-secondary"
            @click="handleExportPDF"
            type="button"
            :disabled="exporting"
          >
            📄 Сохранить как PDF
          </button>
          
          <button
            class="button button-secondary"
            @click="handleExportWord"
            type="button"
            :disabled="exporting"
          >
            📝 Сохранить как Word
          </button>
          
          <button
            class="button button-secondary"
            @click="handleCopy"
            type="button"
            :disabled="exporting"
          >
            📋 Копировать
          </button>
        </div>
      </div>
      
      <div class="print-dialog-footer">
        <button class="button button-secondary" @click="handleClose" type="button">
          Закрыть
        </button>
      </div>
    </div>
  </div>
</template>


