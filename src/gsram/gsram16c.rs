#[doc = "Register `GSRAM16C` reader"]
pub type R = crate::R<Gsram16cSpec>;
#[doc = "Register `GSRAM16C` writer"]
pub type W = crate::W<Gsram16cSpec>;
#[doc = "Field `WLOCK59` reader - WLOCK59"]
pub type Wlock59R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK59` writer - WLOCK59"]
pub type Wlock59W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK59"]
    #[inline(always)]
    pub fn wlock59(&self) -> Wlock59R {
        Wlock59R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK59"]
    #[inline(always)]
    pub fn wlock59(&mut self) -> Wlock59W<Gsram16cSpec> {
        Wlock59W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK59\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram16c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram16c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram16cSpec;
impl crate::RegisterSpec for Gsram16cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram16c::R`](R) reader structure"]
impl crate::Readable for Gsram16cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram16c::W`](W) writer structure"]
impl crate::Writable for Gsram16cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM16C to value 0"]
impl crate::Resettable for Gsram16cSpec {}
