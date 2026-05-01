#[doc = "Register `GSRAM11C` reader"]
pub type R = crate::R<Gsram11cSpec>;
#[doc = "Register `GSRAM11C` writer"]
pub type W = crate::W<Gsram11cSpec>;
#[doc = "Field `WLOCK39` reader - WLOCK39"]
pub type Wlock39R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK39` writer - WLOCK39"]
pub type Wlock39W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK39"]
    #[inline(always)]
    pub fn wlock39(&self) -> Wlock39R {
        Wlock39R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK39"]
    #[inline(always)]
    pub fn wlock39(&mut self) -> Wlock39W<Gsram11cSpec> {
        Wlock39W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK39\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram11cSpec;
impl crate::RegisterSpec for Gsram11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram11c::R`](R) reader structure"]
impl crate::Readable for Gsram11cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram11c::W`](W) writer structure"]
impl crate::Writable for Gsram11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM11C to value 0"]
impl crate::Resettable for Gsram11cSpec {}
