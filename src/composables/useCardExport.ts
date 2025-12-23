import { ref, type Ref } from 'vue'
import html2canvas from 'html2canvas'
import jsPDF from 'jspdf'
import { Document, Packer, Paragraph, TextRun, HeadingLevel, AlignmentType } from 'docx'
import { save } from '@tauri-apps/plugin-dialog'
import { writeFile } from '@tauri-apps/plugin-fs'
import type { ControlCard } from '../types/calendar'

export function useCardExport(printCardRef: Ref<{ $el: HTMLElement } | null>) {
  const exporting = ref(false)

  const formatDate = (dateString?: string): string => {
    if (!dateString) return ''
    try {
      const date = new Date(dateString)
      return date.toLocaleDateString('ru-RU', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      })
    } catch {
      return dateString
    }
  }

  const getFileName = (card: ControlCard, extension: string): string => {
    return `Карточка_${card.cardNumber}_${card.year}.${extension}`
  }

  const printCard = async (card: ControlCard) => {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:printCard',message:'printCard called',data:{cardId:card.id,hasRef:!!printCardRef.value,hasEl:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
    // #endregion
    if (!printCardRef.value?.$el) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:printCard',message:'printCardRef.value.$el is null',data:{hasRef:!!printCardRef.value,refValue:printCardRef.value},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
      // #endregion
      return
    }

    try {
      exporting.value = true
      const cardHtml = printCardRef.value.$el.innerHTML
      const styles = Array.from(document.styleSheets)
        .map(sheet => {
          try {
            return Array.from(sheet.cssRules)
              .map(rule => rule.cssText)
              .join('\n')
          } catch {
            return ''
          }
        })
        .join('\n')

      const printContent = `
        <!DOCTYPE html>
        <html>
          <head>
            <title>Карточка №${card.cardNumber}/${card.year}</title>
            <style>
              ${styles}
              @media print {
                body { margin: 0; padding: 0; }
                @page { size: A4; margin: 15mm; }
              }
            </style>
          </head>
          <body>
            ${cardHtml}
          </body>
        </html>
      `

      const iframe = document.createElement('iframe')
      iframe.style.position = 'fixed'
      iframe.style.right = '0'
      iframe.style.bottom = '0'
      iframe.style.width = '0'
      iframe.style.height = '0'
      iframe.style.border = 'none'
      document.body.appendChild(iframe)

      const iframeDoc = iframe.contentDocument || iframe.contentWindow?.document
      if (iframeDoc) {
        iframeDoc.open()
        iframeDoc.write(printContent)
        iframeDoc.close()
        
        // #region agent log
        fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:printCard',message:'iframe created, calling print',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
        // #endregion
        
        setTimeout(() => {
          iframe.contentWindow?.print()
          setTimeout(() => {
            document.body.removeChild(iframe)
          }, 1000)
        }, 250)
      }
    } catch (error) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:printCard',message:'Error in printCard',data:{errorMessage:error instanceof Error?error.message:String(error)},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      console.error('Ошибка печати:', error)
      alert('Ошибка при печати')
    } finally {
      exporting.value = false
    }
  }

  const exportAsImage = async (card: ControlCard) => {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'exportAsImage called',data:{cardId:card.id,hasRef:!!printCardRef.value,hasEl:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
    // #endregion
    if (!printCardRef.value?.$el) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'printCardRef.value.$el is null',data:{hasRef:!!printCardRef.value},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
      // #endregion
      return
    }

    try {
      exporting.value = true
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'Starting html2canvas',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      const canvas = await html2canvas(printCardRef.value.$el, {
        backgroundColor: '#ffffff',
        scale: 2,
        logging: false,
        useCORS: true
      })
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'html2canvas completed',data:{canvasWidth:canvas.width,canvasHeight:canvas.height},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion

      const blob = await new Promise<Blob | null>((resolve) => {
        canvas.toBlob((blob) => {
          resolve(blob)
        }, 'image/png')
      })

      if (!blob) {
        // #region agent log
        fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'Blob is null',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
        // #endregion
        throw new Error('Failed to create image blob')
      }

      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'Blob created, opening save dialog',data:{blobSize:blob.size},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion

      const filePath = await save({
        defaultPath: getFileName(card, 'png'),
        filters: [{ name: 'PNG Image', extensions: ['png'] }]
      })

      if (filePath) {
        const arrayBuffer = await blob.arrayBuffer()
        const uint8Array = new Uint8Array(arrayBuffer)
        await writeFile(filePath, uint8Array)
        // #region agent log
        fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'File saved successfully',data:{filePath},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
        // #endregion
      }
    } catch (error) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsImage',message:'Error in exportAsImage',data:{errorMessage:error instanceof Error?error.message:String(error),errorStack:error instanceof Error?error.stack:undefined},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      console.error('Ошибка экспорта изображения:', error)
      alert('Ошибка при сохранении изображения')
    } finally {
      exporting.value = false
    }
  }

  const exportAsPDF = async (card: ControlCard) => {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsPDF',message:'exportAsPDF called',data:{cardId:card.id,hasRef:!!printCardRef.value,hasEl:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
    // #endregion
    if (!printCardRef.value?.$el) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsPDF',message:'printCardRef.value.$el is null',data:{hasRef:!!printCardRef.value},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
      // #endregion
      return
    }

    try {
      exporting.value = true
      const canvas = await html2canvas(printCardRef.value.$el, {
        backgroundColor: '#ffffff',
        scale: 2,
        logging: false,
        useCORS: true
      })

      const imgData = canvas.toDataURL('image/png')
      const pdf = new jsPDF({
        orientation: 'portrait',
        unit: 'mm',
        format: 'a4'
      })

      const pdfWidth = pdf.internal.pageSize.getWidth()
      const pdfHeight = pdf.internal.pageSize.getHeight()
      const imgWidth = canvas.width
      const imgHeight = canvas.height
      const ratio = Math.min((pdfWidth - 20) / imgWidth, (pdfHeight - 20) / imgHeight)
      const imgX = (pdfWidth - imgWidth * ratio) / 2
      const imgY = 10

      pdf.addImage(imgData, 'PNG', imgX, imgY, imgWidth * ratio, imgHeight * ratio)
      
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsPDF',message:'PDF created, opening save dialog',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion

      const pdfBlob = pdf.output('blob')
      const filePath = await save({
        defaultPath: getFileName(card, 'pdf'),
        filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
      })

      if (filePath) {
        const arrayBuffer = await pdfBlob.arrayBuffer()
        const uint8Array = new Uint8Array(arrayBuffer)
        await writeFile(filePath, uint8Array)
        // #region agent log
        fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsPDF',message:'PDF saved successfully',data:{filePath},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
        // #endregion
      }
    } catch (error) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsPDF',message:'Error in exportAsPDF',data:{errorMessage:error instanceof Error?error.message:String(error),errorStack:error instanceof Error?error.stack:undefined},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      console.error('Ошибка экспорта PDF:', error)
      alert('Ошибка при сохранении PDF')
    } finally {
      exporting.value = false
    }
  }

  const exportAsWord = async (card: ControlCard) => {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsWord',message:'exportAsWord called',data:{cardId:card.id},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
    // #endregion
    try {
      exporting.value = true

      const doc = new Document({
        sections: [{
          properties: {},
          children: [
            // Заголовок карточки
            new Paragraph({
              text: `№${card.cardNumber}/${card.year}`,
              heading: HeadingLevel.HEADING_1,
              alignment: AlignmentType.LEFT,
              spacing: { after: 400 },
              children: [
                new TextRun({
                  text: `№${card.cardNumber}/${card.year}`,
                  size: 32,
                  font: 'Arial',
                  bold: true
                })
              ]
            }),
            // Поле "Исполнитель"
            new Paragraph({
              children: [
                new TextRun({
                  text: 'Исполнитель',
                  bold: true,
                  color: '6B7280',
                  size: 22
                })
              ],
              spacing: { after: 100 }
            }),
            new Paragraph({
              text: card.executor,
              spacing: { after: 300 }
            }),
            // Поле "Кому докладывать"
            new Paragraph({
              children: [
                new TextRun({
                  text: 'Кому докладывать',
                  bold: true,
                  color: '6B7280',
                  size: 22
                })
              ],
              spacing: { after: 100 }
            }),
            new Paragraph({
              text: card.reporter,
              spacing: { after: 300 }
            }),
            // Поле "Краткое содержание"
            new Paragraph({
              children: [
                new TextRun({
                  text: 'Краткое содержание',
                  bold: true,
                  color: '6B7280',
                  size: 22
                })
              ],
              spacing: { after: 100 }
            }),
            new Paragraph({
              text: card.summary,
              spacing: { after: 300 }
            }),
            // Поле "Документ-основание"
            new Paragraph({
              children: [
                new TextRun({
                  text: 'Документ-основание',
                  bold: true,
                  color: '6B7280',
                  size: 22
                })
              ],
              spacing: { after: 100 }
            }),
            new Paragraph({
              text: card.documentReference,
              spacing: { after: 300 }
            }),
            // Поле "Дата создания" (если есть)
            ...(card.createdAt ? [
              new Paragraph({
                children: [
                  new TextRun({
                    text: 'Дата создания',
                    bold: true,
                    color: '6B7280',
                    size: 22
                  })
                ],
                spacing: { after: 100 }
              }),
              new Paragraph({
                text: formatDate(card.createdAt),
                spacing: { after: 200 }
              })
            ] : [])
          ]
        }]
      })

      const blob = await Packer.toBlob(doc)
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsWord',message:'Packer.toBlob completed, opening save dialog',data:{blobSize:blob.size},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion

      const filePath = await save({
        defaultPath: getFileName(card, 'docx'),
        filters: [{ name: 'Word Document', extensions: ['docx'] }]
      })

      if (filePath) {
        const arrayBuffer = await blob.arrayBuffer()
        const uint8Array = new Uint8Array(arrayBuffer)
        await writeFile(filePath, uint8Array)
        // #region agent log
        fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsWord',message:'Word document saved successfully',data:{filePath},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
        // #endregion
      }
    } catch (error) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:exportAsWord',message:'Error in exportAsWord',data:{errorMessage:error instanceof Error?error.message:String(error),errorStack:error instanceof Error?error.stack:undefined},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      console.error('Ошибка экспорта Word:', error)
      alert('Ошибка при сохранении Word документа')
    } finally {
      exporting.value = false
    }
  }

  const copyToClipboard = async (card: ControlCard) => {
    // #region agent log
    fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:copyToClipboard',message:'copyToClipboard called',data:{cardId:card.id,hasRef:!!printCardRef.value,hasEl:!!printCardRef.value?.$el},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
    // #endregion
    if (!printCardRef.value?.$el) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:copyToClipboard',message:'printCardRef.value.$el is null',data:{hasRef:!!printCardRef.value},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'A'})}).catch(()=>{});
      // #endregion
      return
    }

    try {
      exporting.value = true
      const htmlContent = printCardRef.value.$el.innerHTML
      const textContent = `
Контрольная карточка №${card.cardNumber}/${card.year}

Исполнитель: ${card.executor}
Кому докладывать: ${card.reporter}
Краткое содержание: ${card.summary}
Документ-основание: ${card.documentReference}
${card.createdAt ? `Дата создания: ${formatDate(card.createdAt)}` : ''}
      `.trim()

      const clipboardItem = new ClipboardItem({
        'text/html': new Blob([htmlContent], { type: 'text/html' }),
        'text/plain': new Blob([textContent], { type: 'text/plain' })
      })

      await navigator.clipboard.write([clipboardItem])
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:copyToClipboard',message:'Clipboard write successful',data:{},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      alert('Карточка скопирована в буфер обмена')
    } catch (error) {
      // #region agent log
      fetch('http://127.0.0.1:7244/ingest/8366886c-c43b-42f3-87a2-defecea0a34d',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'useCardExport.ts:copyToClipboard',message:'Error in copyToClipboard',data:{errorMessage:error instanceof Error?error.message:String(error),errorStack:error instanceof Error?error.stack:undefined},timestamp:Date.now(),sessionId:'debug-session',runId:'post-fix',hypothesisId:'C'})}).catch(()=>{});
      // #endregion
      console.error('Ошибка копирования:', error)
      const textContent = `
Контрольная карточка №${card.cardNumber}/${card.year}

Исполнитель: ${card.executor}
Кому докладывать: ${card.reporter}
Краткое содержание: ${card.summary}
Документ-основание: ${card.documentReference}
${card.createdAt ? `Дата создания: ${formatDate(card.createdAt)}` : ''}
      `.trim()
      
      try {
        await navigator.clipboard.writeText(textContent)
        alert('Карточка скопирована в буфер обмена')
      } catch (err) {
        alert('Ошибка при копировании в буфер обмена')
      }
    } finally {
      exporting.value = false
    }
  }

  return {
    exporting,
    printCard,
    exportAsImage,
    exportAsPDF,
    exportAsWord,
    copyToClipboard
  }
}

