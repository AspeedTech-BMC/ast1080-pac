#[doc = "Register `SPROT010` reader"]
pub type R = crate::R<Sprot010Spec>;
#[doc = "Register `SPROT010` writer"]
pub type W = crate::W<Sprot010Spec>;
#[doc = "Field `SID00` reader - SID00"]
pub type Sid00R = crate::FieldReader;
#[doc = "Field `SID00` writer - SID00"]
pub type Sid00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SID01` reader - SID01"]
pub type Sid01R = crate::FieldReader;
#[doc = "Field `SID01` writer - SID01"]
pub type Sid01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SID02` reader - SID02"]
pub type Sid02R = crate::FieldReader;
#[doc = "Field `SID02` writer - SID02"]
pub type Sid02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SID03` reader - SID03"]
pub type Sid03R = crate::FieldReader;
#[doc = "Field `SID03` writer - SID03"]
pub type Sid03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SID00"]
    #[inline(always)]
    pub fn sid00(&self) -> Sid00R {
        Sid00R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SID01"]
    #[inline(always)]
    pub fn sid01(&self) -> Sid01R {
        Sid01R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SID02"]
    #[inline(always)]
    pub fn sid02(&self) -> Sid02R {
        Sid02R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SID03"]
    #[inline(always)]
    pub fn sid03(&self) -> Sid03R {
        Sid03R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SID00"]
    #[inline(always)]
    pub fn sid00(&mut self) -> Sid00W<Sprot010Spec> {
        Sid00W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SID01"]
    #[inline(always)]
    pub fn sid01(&mut self) -> Sid01W<Sprot010Spec> {
        Sid01W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SID02"]
    #[inline(always)]
    pub fn sid02(&mut self) -> Sid02W<Sprot010Spec> {
        Sid02W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SID03"]
    #[inline(always)]
    pub fn sid03(&mut self) -> Sid03W<Sprot010Spec> {
        Sid03W::new(self, 24)
    }
}
#[doc = "SPROT\\_SIDG0\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot010Spec;
impl crate::RegisterSpec for Sprot010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot010::R`](R) reader structure"]
impl crate::Readable for Sprot010Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot010::W`](W) writer structure"]
impl crate::Writable for Sprot010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT010 to value 0"]
impl crate::Resettable for Sprot010Spec {}
