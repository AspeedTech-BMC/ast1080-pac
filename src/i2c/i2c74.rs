#[doc = "Register `I2C74` reader"]
pub type R = crate::R<I2c74Spec>;
#[doc = "Register `I2C74` writer"]
pub type W = crate::W<I2c74Spec>;
#[doc = "Field `MTIMEOUT` reader - MTIMEOUT"]
pub type MtimeoutR = crate::FieldReader;
#[doc = "Field `MTIMEOUT` writer - MTIMEOUT"]
pub type MtimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `STIMEOUT` reader - STIMEOUT"]
pub type StimeoutR = crate::FieldReader;
#[doc = "Field `STIMEOUT` writer - STIMEOUT"]
pub type StimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MTIMEOUT"]
    #[inline(always)]
    pub fn mtimeout(&self) -> MtimeoutR {
        MtimeoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - STIMEOUT"]
    #[inline(always)]
    pub fn stimeout(&self) -> StimeoutR {
        StimeoutR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MTIMEOUT"]
    #[inline(always)]
    pub fn mtimeout(&mut self) -> MtimeoutW<I2c74Spec> {
        MtimeoutW::new(self, 0)
    }
    #[doc = "Bits 16:23 - STIMEOUT"]
    #[inline(always)]
    pub fn stimeout(&mut self) -> StimeoutW<I2c74Spec> {
        StimeoutW::new(self, 16)
    }
}
#[doc = "MISC configuration for AC timing1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c74Spec;
impl crate::RegisterSpec for I2c74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c74::R`](R) reader structure"]
impl crate::Readable for I2c74Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c74::W`](W) writer structure"]
impl crate::Writable for I2c74Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C74 to value 0"]
impl crate::Resettable for I2c74Spec {}
