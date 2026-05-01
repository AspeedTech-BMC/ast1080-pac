#[doc = "Register `I2C0C` reader"]
pub type R = crate::R<I2c0cSpec>;
#[doc = "Register `I2C0C` writer"]
pub type W = crate::W<I2c0cSpec>;
#[doc = "Field `BUFCTL` reader - BUF_CTL"]
pub type BufctlR = crate::FieldReader<u32>;
#[doc = "Field `BUFCTL` writer - BUF_CTL"]
pub type BufctlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BUF_CTL"]
    #[inline(always)]
    pub fn bufctl(&self) -> BufctlR {
        BufctlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BUF_CTL"]
    #[inline(always)]
    pub fn bufctl(&mut self) -> BufctlW<I2c0cSpec> {
        BufctlW::new(self, 0)
    }
}
#[doc = "I2CC\\_BUFCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0cSpec;
impl crate::RegisterSpec for I2c0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0c::R`](R) reader structure"]
impl crate::Readable for I2c0cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0c::W`](W) writer structure"]
impl crate::Writable for I2c0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0C to value 0"]
impl crate::Resettable for I2c0cSpec {}
