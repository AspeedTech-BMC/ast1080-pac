#[doc = "Register `SCU688` reader"]
pub type R = crate::R<Scu688Spec>;
#[doc = "Register `SCU688` writer"]
pub type W = crate::W<Scu688Spec>;
#[doc = "Field `SCUIOCTRLDQ0` reader - SCU_IO_CTRL_DQ0"]
pub type Scuioctrldq0R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ0` writer - SCU_IO_CTRL_DQ0"]
pub type Scuioctrldq0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ1` reader - SCU_IO_CTRL_DQ1"]
pub type Scuioctrldq1R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ1` writer - SCU_IO_CTRL_DQ1"]
pub type Scuioctrldq1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ2` reader - SCU_IO_CTRL_DQ2"]
pub type Scuioctrldq2R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ2` writer - SCU_IO_CTRL_DQ2"]
pub type Scuioctrldq2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ3` reader - SCU_IO_CTRL_DQ3"]
pub type Scuioctrldq3R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ3` writer - SCU_IO_CTRL_DQ3"]
pub type Scuioctrldq3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ4` reader - SCU_IO_CTRL_DQ4"]
pub type Scuioctrldq4R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ4` writer - SCU_IO_CTRL_DQ4"]
pub type Scuioctrldq4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ5` reader - SCU_IO_CTRL_DQ5"]
pub type Scuioctrldq5R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ5` writer - SCU_IO_CTRL_DQ5"]
pub type Scuioctrldq5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ6` reader - SCU_IO_CTRL_DQ6"]
pub type Scuioctrldq6R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ6` writer - SCU_IO_CTRL_DQ6"]
pub type Scuioctrldq6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLDQ7` reader - SCU_IO_CTRL_DQ7"]
pub type Scuioctrldq7R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLDQ7` writer - SCU_IO_CTRL_DQ7"]
pub type Scuioctrldq7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_DQ0"]
    #[inline(always)]
    pub fn scuioctrldq0(&self) -> Scuioctrldq0R {
        Scuioctrldq0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_DQ1"]
    #[inline(always)]
    pub fn scuioctrldq1(&self) -> Scuioctrldq1R {
        Scuioctrldq1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_DQ2"]
    #[inline(always)]
    pub fn scuioctrldq2(&self) -> Scuioctrldq2R {
        Scuioctrldq2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_DQ3"]
    #[inline(always)]
    pub fn scuioctrldq3(&self) -> Scuioctrldq3R {
        Scuioctrldq3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SCU_IO_CTRL_DQ4"]
    #[inline(always)]
    pub fn scuioctrldq4(&self) -> Scuioctrldq4R {
        Scuioctrldq4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCU_IO_CTRL_DQ5"]
    #[inline(always)]
    pub fn scuioctrldq5(&self) -> Scuioctrldq5R {
        Scuioctrldq5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SCU_IO_CTRL_DQ6"]
    #[inline(always)]
    pub fn scuioctrldq6(&self) -> Scuioctrldq6R {
        Scuioctrldq6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SCU_IO_CTRL_DQ7"]
    #[inline(always)]
    pub fn scuioctrldq7(&self) -> Scuioctrldq7R {
        Scuioctrldq7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_DQ0"]
    #[inline(always)]
    pub fn scuioctrldq0(&mut self) -> Scuioctrldq0W<Scu688Spec> {
        Scuioctrldq0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_DQ1"]
    #[inline(always)]
    pub fn scuioctrldq1(&mut self) -> Scuioctrldq1W<Scu688Spec> {
        Scuioctrldq1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_DQ2"]
    #[inline(always)]
    pub fn scuioctrldq2(&mut self) -> Scuioctrldq2W<Scu688Spec> {
        Scuioctrldq2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_DQ3"]
    #[inline(always)]
    pub fn scuioctrldq3(&mut self) -> Scuioctrldq3W<Scu688Spec> {
        Scuioctrldq3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - SCU_IO_CTRL_DQ4"]
    #[inline(always)]
    pub fn scuioctrldq4(&mut self) -> Scuioctrldq4W<Scu688Spec> {
        Scuioctrldq4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SCU_IO_CTRL_DQ5"]
    #[inline(always)]
    pub fn scuioctrldq5(&mut self) -> Scuioctrldq5W<Scu688Spec> {
        Scuioctrldq5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - SCU_IO_CTRL_DQ6"]
    #[inline(always)]
    pub fn scuioctrldq6(&mut self) -> Scuioctrldq6W<Scu688Spec> {
        Scuioctrldq6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - SCU_IO_CTRL_DQ7"]
    #[inline(always)]
    pub fn scuioctrldq7(&mut self) -> Scuioctrldq7W<Scu688Spec> {
        Scuioctrldq7W::new(self, 28)
    }
}
#[doc = "HRAM IO Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu688::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu688::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu688Spec;
impl crate::RegisterSpec for Scu688Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu688::R`](R) reader structure"]
impl crate::Readable for Scu688Spec {}
#[doc = "`write(|w| ..)` method takes [`scu688::W`](W) writer structure"]
impl crate::Writable for Scu688Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU688 to value 0x1111_1111"]
impl crate::Resettable for Scu688Spec {
    const RESET_VALUE: u32 = 0x1111_1111;
}
