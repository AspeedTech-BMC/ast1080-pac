#[doc = "Register `I2C48` reader"]
pub type R = crate::R<I2c48Spec>;
#[doc = "Register `I2C48` writer"]
pub type W = crate::W<I2c48Spec>;
#[doc = "Field `MTXLENACT` reader - MTXLEN_ACT"]
pub type MtxlenactR = crate::FieldReader<u16>;
#[doc = "Field `MTXLENACT` writer - MTXLEN_ACT"]
pub type MtxlenactW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRXLENACT` reader - MRXLEN_ACT"]
pub type MrxlenactR = crate::FieldReader<u16>;
#[doc = "Field `MRXLENACT` writer - MRXLEN_ACT"]
pub type MrxlenactW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MTXLEN_ACT"]
    #[inline(always)]
    pub fn mtxlenact(&self) -> MtxlenactR {
        MtxlenactR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRXLEN_ACT"]
    #[inline(always)]
    pub fn mrxlenact(&self) -> MrxlenactR {
        MrxlenactR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MTXLEN_ACT"]
    #[inline(always)]
    pub fn mtxlenact(&mut self) -> MtxlenactW<I2c48Spec> {
        MtxlenactW::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRXLEN_ACT"]
    #[inline(always)]
    pub fn mrxlenact(&mut self) -> MrxlenactW<I2c48Spec> {
        MrxlenactW::new(self, 16)
    }
}
#[doc = "Master DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c48Spec;
impl crate::RegisterSpec for I2c48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c48::R`](R) reader structure"]
impl crate::Readable for I2c48Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c48::W`](W) writer structure"]
impl crate::Writable for I2c48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C48 to value 0"]
impl crate::Resettable for I2c48Spec {}
