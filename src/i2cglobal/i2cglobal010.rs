#[doc = "Register `I2CGLOBAL010` reader"]
pub type R = crate::R<I2cglobal010Spec>;
#[doc = "Register `I2CGLOBAL010` writer"]
pub type W = crate::W<I2cglobal010Spec>;
#[doc = "Field `BASEDIV1` reader - BASE_DIV1"]
pub type Basediv1R = crate::FieldReader;
#[doc = "Field `BASEDIV1` writer - BASE_DIV1"]
pub type Basediv1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BASEDIV2` reader - BASE_DIV2"]
pub type Basediv2R = crate::FieldReader;
#[doc = "Field `BASEDIV2` writer - BASE_DIV2"]
pub type Basediv2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BASEDIV3` reader - BASE_DIV3"]
pub type Basediv3R = crate::FieldReader;
#[doc = "Field `BASEDIV3` writer - BASE_DIV3"]
pub type Basediv3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BASEDIV4` reader - BASE_DIV4"]
pub type Basediv4R = crate::FieldReader;
#[doc = "Field `BASEDIV4` writer - BASE_DIV4"]
pub type Basediv4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BASE_DIV1"]
    #[inline(always)]
    pub fn basediv1(&self) -> Basediv1R {
        Basediv1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BASE_DIV2"]
    #[inline(always)]
    pub fn basediv2(&self) -> Basediv2R {
        Basediv2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BASE_DIV3"]
    #[inline(always)]
    pub fn basediv3(&self) -> Basediv3R {
        Basediv3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - BASE_DIV4"]
    #[inline(always)]
    pub fn basediv4(&self) -> Basediv4R {
        Basediv4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BASE_DIV1"]
    #[inline(always)]
    pub fn basediv1(&mut self) -> Basediv1W<I2cglobal010Spec> {
        Basediv1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - BASE_DIV2"]
    #[inline(always)]
    pub fn basediv2(&mut self) -> Basediv2W<I2cglobal010Spec> {
        Basediv2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - BASE_DIV3"]
    #[inline(always)]
    pub fn basediv3(&mut self) -> Basediv3W<I2cglobal010Spec> {
        Basediv3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - BASE_DIV4"]
    #[inline(always)]
    pub fn basediv4(&mut self) -> Basediv4W<I2cglobal010Spec> {
        Basediv4W::new(self, 24)
    }
}
#[doc = "New Clock Divider Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cglobal010Spec;
impl crate::RegisterSpec for I2cglobal010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cglobal010::R`](R) reader structure"]
impl crate::Readable for I2cglobal010Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cglobal010::W`](W) writer structure"]
impl crate::Writable for I2cglobal010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CGLOBAL010 to value 0"]
impl crate::Resettable for I2cglobal010Spec {}
