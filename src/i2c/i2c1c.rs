#[doc = "Register `I2C1C` reader"]
pub type R = crate::R<I2c1cSpec>;
#[doc = "Register `I2C1C` writer"]
pub type W = crate::W<I2c1cSpec>;
#[doc = "Field `MTXLEN` reader - MTXLEN"]
pub type MtxlenR = crate::FieldReader<u16>;
#[doc = "Field `MTXLEN` writer - MTXLEN"]
pub type MtxlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRXLEN` reader - MRXLEN"]
pub type MrxlenR = crate::FieldReader<u16>;
#[doc = "Field `MRXLEN` writer - MRXLEN"]
pub type MrxlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MTXLEN"]
    #[inline(always)]
    pub fn mtxlen(&self) -> MtxlenR {
        MtxlenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRXLEN"]
    #[inline(always)]
    pub fn mrxlen(&self) -> MrxlenR {
        MrxlenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MTXLEN"]
    #[inline(always)]
    pub fn mtxlen(&mut self) -> MtxlenW<I2c1cSpec> {
        MtxlenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRXLEN"]
    #[inline(always)]
    pub fn mrxlen(&mut self) -> MrxlenW<I2c1cSpec> {
        MrxlenW::new(self, 16)
    }
}
#[doc = "Master DMA Buffer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1cSpec;
impl crate::RegisterSpec for I2c1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1c::R`](R) reader structure"]
impl crate::Readable for I2c1cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1c::W`](W) writer structure"]
impl crate::Writable for I2c1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C1C to value 0"]
impl crate::Resettable for I2c1cSpec {}
