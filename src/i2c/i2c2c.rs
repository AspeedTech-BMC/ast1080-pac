#[doc = "Register `I2C2C` reader"]
pub type R = crate::R<I2c2cSpec>;
#[doc = "Register `I2C2C` writer"]
pub type W = crate::W<I2c2cSpec>;
#[doc = "Field `STXLEN` reader - STXLEN"]
pub type StxlenR = crate::FieldReader<u16>;
#[doc = "Field `STXLEN` writer - STXLEN"]
pub type StxlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRXLEN` reader - SRXLEN"]
pub type SrxlenR = crate::FieldReader<u16>;
#[doc = "Field `SRXLEN` writer - SRXLEN"]
pub type SrxlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - STXLEN"]
    #[inline(always)]
    pub fn stxlen(&self) -> StxlenR {
        StxlenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRXLEN"]
    #[inline(always)]
    pub fn srxlen(&self) -> SrxlenR {
        SrxlenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - STXLEN"]
    #[inline(always)]
    pub fn stxlen(&mut self) -> StxlenW<I2c2cSpec> {
        StxlenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRXLEN"]
    #[inline(always)]
    pub fn srxlen(&mut self) -> SrxlenW<I2c2cSpec> {
        SrxlenW::new(self, 16)
    }
}
#[doc = "Slave DMA Buffer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2cSpec;
impl crate::RegisterSpec for I2c2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2c::R`](R) reader structure"]
impl crate::Readable for I2c2cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c2c::W`](W) writer structure"]
impl crate::Writable for I2c2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2C to value 0"]
impl crate::Resettable for I2c2cSpec {}
