#[doc = "Register `I2C54` reader"]
pub type R = crate::R<I2c54Spec>;
#[doc = "Register `I2C54` writer"]
pub type W = crate::W<I2c54Spec>;
#[doc = "Field `WDMAEN` reader - WDMA_EN"]
pub type WdmaenR = crate::BitReader;
#[doc = "Field `WDMAEN` writer - WDMA_EN"]
pub type WdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDMACLR` reader - WDMA_CLR"]
pub type WdmaclrR = crate::BitReader;
#[doc = "Field `WDMACLR` writer - WDMA_CLR"]
pub type WdmaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `RDMAEN` reader - RDMA_EN"]
pub type RdmaenR = crate::BitReader;
#[doc = "Field `RDMAEN` writer - RDMA_EN"]
pub type RdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMACLR` reader - RDMA_CLR"]
pub type RdmaclrR = crate::BitReader;
#[doc = "Field `RDMACLR` writer - RDMA_CLR"]
pub type RdmaclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDMA_EN"]
    #[inline(always)]
    pub fn wdmaen(&self) -> WdmaenR {
        WdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDMA_CLR"]
    #[inline(always)]
    pub fn wdmaclr(&self) -> WdmaclrR {
        WdmaclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - RDMA_EN"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RdmaenR {
        RdmaenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RDMA_CLR"]
    #[inline(always)]
    pub fn rdmaclr(&self) -> RdmaclrR {
        RdmaclrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDMA_EN"]
    #[inline(always)]
    pub fn wdmaen(&mut self) -> WdmaenW<I2c54Spec> {
        WdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - WDMA_CLR"]
    #[inline(always)]
    pub fn wdmaclr(&mut self) -> WdmaclrW<I2c54Spec> {
        WdmaclrW::new(self, 1)
    }
    #[doc = "Bit 16 - RDMA_EN"]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RdmaenW<I2c54Spec> {
        RdmaenW::new(self, 16)
    }
    #[doc = "Bit 17 - RDMA_CLR"]
    #[inline(always)]
    pub fn rdmaclr(&mut self) -> RdmaclrW<I2c54Spec> {
        RdmaclrW::new(self, 17)
    }
}
#[doc = "I2CC\\_DMA\\_STA\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c54Spec;
impl crate::RegisterSpec for I2c54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c54::R`](R) reader structure"]
impl crate::Readable for I2c54Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c54::W`](W) writer structure"]
impl crate::Writable for I2c54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C54 to value 0"]
impl crate::Resettable for I2c54Spec {}
