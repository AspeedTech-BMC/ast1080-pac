#[doc = "Register `SCUC5C` reader"]
pub type R = crate::R<Scuc5cSpec>;
#[doc = "Register `SCUC5C` writer"]
pub type W = crate::W<Scuc5cSpec>;
#[doc = "Field `SCUREGSEC1B80` reader - SCU_REG_SEC1_B80"]
pub type Scuregsec1b80R = crate::BitReader;
#[doc = "Field `SCUREGSEC1B80` writer - SCU_REG_SEC1_B80"]
pub type Scuregsec1b80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC1_B80"]
    #[inline(always)]
    pub fn scuregsec1b80(&self) -> Scuregsec1b80R {
        Scuregsec1b80R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC1_B80"]
    #[inline(always)]
    pub fn scuregsec1b80(&mut self) -> Scuregsec1b80W<Scuc5cSpec> {
        Scuregsec1b80W::new(self, 0)
    }
}
#[doc = "Secure1 Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc5cSpec;
impl crate::RegisterSpec for Scuc5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc5c::R`](R) reader structure"]
impl crate::Readable for Scuc5cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuc5c::W`](W) writer structure"]
impl crate::Writable for Scuc5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC5C to value 0"]
impl crate::Resettable for Scuc5cSpec {}
