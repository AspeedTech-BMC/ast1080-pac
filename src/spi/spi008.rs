#[doc = "Register `SPI008` reader"]
pub type R = crate::R<Spi008Spec>;
#[doc = "Register `SPI008` writer"]
pub type W = crate::W<Spi008Spec>;
#[doc = "Field `IRQEN` reader - IRQ_EN"]
pub type IrqenR = crate::FieldReader;
#[doc = "Field `IRQEN` writer - IRQ_EN"]
pub type IrqenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `IRQDMA` reader - IRQ_DMA"]
pub type IrqdmaR = crate::BitReader;
#[doc = "Field `DMAFIFOEMPTY` reader - DMA_FIFO_EMPTY"]
pub type DmafifoemptyR = crate::BitReader;
#[doc = "Field `DMAFIFOFULL` reader - DMA_FIFO_FULL"]
pub type DmafifofullR = crate::BitReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `IRQSTA` reader - IRQ_STA"]
pub type IrqstaR = crate::FieldReader;
#[doc = "Field `IRQSTA` writer - IRQ_STA"]
pub type IrqstaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IRQ_EN"]
    #[inline(always)]
    pub fn irqen(&self) -> IrqenR {
        IrqenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IRQ_DMA"]
    #[inline(always)]
    pub fn irqdma(&self) -> IrqdmaR {
        IrqdmaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA_FIFO_EMPTY"]
    #[inline(always)]
    pub fn dmafifoempty(&self) -> DmafifoemptyR {
        DmafifoemptyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA_FIFO_FULL"]
    #[inline(always)]
    pub fn dmafifofull(&self) -> DmafifofullR {
        DmafifofullR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - IRQ_STA"]
    #[inline(always)]
    pub fn irqsta(&self) -> IrqstaR {
        IrqstaR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IRQ_EN"]
    #[inline(always)]
    pub fn irqen(&mut self) -> IrqenW<Spi008Spec> {
        IrqenW::new(self, 0)
    }
    #[doc = "Bits 16:23 - IRQ_STA"]
    #[inline(always)]
    pub fn irqsta(&mut self) -> IrqstaW<Spi008Spec> {
        IrqstaW::new(self, 16)
    }
}
#[doc = "Interrupt Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi008Spec;
impl crate::RegisterSpec for Spi008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi008::R`](R) reader structure"]
impl crate::Readable for Spi008Spec {}
#[doc = "`write(|w| ..)` method takes [`spi008::W`](W) writer structure"]
impl crate::Writable for Spi008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI008 to value 0"]
impl crate::Resettable for Spi008Spec {}
