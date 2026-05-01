#[doc = "Register `GSRAM10C` reader"]
pub type R = crate::R<Gsram10cSpec>;
#[doc = "Register `GSRAM10C` writer"]
pub type W = crate::W<Gsram10cSpec>;
#[doc = "Field `WLOCK35` reader - WLOCK35"]
pub type Wlock35R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK35` writer - WLOCK35"]
pub type Wlock35W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK35"]
    #[inline(always)]
    pub fn wlock35(&self) -> Wlock35R {
        Wlock35R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK35"]
    #[inline(always)]
    pub fn wlock35(&mut self) -> Wlock35W<Gsram10cSpec> {
        Wlock35W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK35\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram10cSpec;
impl crate::RegisterSpec for Gsram10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram10c::R`](R) reader structure"]
impl crate::Readable for Gsram10cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram10c::W`](W) writer structure"]
impl crate::Writable for Gsram10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM10C to value 0"]
impl crate::Resettable for Gsram10cSpec {}
