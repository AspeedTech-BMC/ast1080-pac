#[doc = "Register `SPROT0DC` reader"]
pub type R = crate::R<Sprot0dcSpec>;
#[doc = "Register `SPROT0DC` writer"]
pub type W = crate::W<Sprot0dcSpec>;
#[doc = "Field `SRNGSADR07` reader - SRNG_SADR07"]
pub type Srngsadr07R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR07` writer - SRNG_SADR07"]
pub type Srngsadr07W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE07` reader - SRNG_SIZE07"]
pub type Srngsize07R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE07` writer - SRNG_SIZE07"]
pub type Srngsize07W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR07"]
    #[inline(always)]
    pub fn srngsadr07(&self) -> Srngsadr07R {
        Srngsadr07R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE07"]
    #[inline(always)]
    pub fn srngsize07(&self) -> Srngsize07R {
        Srngsize07R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR07"]
    #[inline(always)]
    pub fn srngsadr07(&mut self) -> Srngsadr07W<Sprot0dcSpec> {
        Srngsadr07W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE07"]
    #[inline(always)]
    pub fn srngsize07(&mut self) -> Srngsize07W<Sprot0dcSpec> {
        Srngsize07W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR07\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0dcSpec;
impl crate::RegisterSpec for Sprot0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0dc::R`](R) reader structure"]
impl crate::Readable for Sprot0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot0dc::W`](W) writer structure"]
impl crate::Writable for Sprot0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0DC to value 0"]
impl crate::Resettable for Sprot0dcSpec {}
