#[doc = "Register `I2C4C` reader"]
pub type R = crate::R<I2c4cSpec>;
#[doc = "Register `I2C4C` writer"]
pub type W = crate::W<I2c4cSpec>;
#[doc = "Field `STXLENACT` reader - STXLEN_ACT"]
pub type StxlenactR = crate::FieldReader<u16>;
#[doc = "Field `STXLENACT` writer - STXLEN_ACT"]
pub type StxlenactW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRXLENACT` reader - SRXLEN_ACT"]
pub type SrxlenactR = crate::FieldReader<u16>;
#[doc = "Field `SRXLENACT` writer - SRXLEN_ACT"]
pub type SrxlenactW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - STXLEN_ACT"]
    #[inline(always)]
    pub fn stxlenact(&self) -> StxlenactR {
        StxlenactR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRXLEN_ACT"]
    #[inline(always)]
    pub fn srxlenact(&self) -> SrxlenactR {
        SrxlenactR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - STXLEN_ACT"]
    #[inline(always)]
    pub fn stxlenact(&mut self) -> StxlenactW<I2c4cSpec> {
        StxlenactW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRXLEN_ACT"]
    #[inline(always)]
    pub fn srxlenact(&mut self) -> SrxlenactW<I2c4cSpec> {
        SrxlenactW::new(self, 16)
    }
}
#[doc = "Slave DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c4cSpec;
impl crate::RegisterSpec for I2c4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c4c::R`](R) reader structure"]
impl crate::Readable for I2c4cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c4c::W`](W) writer structure"]
impl crate::Writable for I2c4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C4C to value 0"]
impl crate::Resettable for I2c4cSpec {}
