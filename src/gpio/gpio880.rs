#[doc = "Register `GPIO880` reader"]
pub type R = crate::R<Gpio880Spec>;
#[doc = "Register `GPIO880` writer"]
pub type W = crate::W<Gpio880Spec>;
#[doc = "Field `GPIO112WrPrivilegeOfMaster` reader - GPIO112 Write Privilege of Master"]
pub type Gpio112wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO112WrPrivilegeOfMaster` writer - GPIO112 Write Privilege of Master"]
pub type Gpio112wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO113WrPrivilegeOfMaster` reader - GPIO113 Write Privilege of Master"]
pub type Gpio113wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO113WrPrivilegeOfMaster` writer - GPIO113 Write Privilege of Master"]
pub type Gpio113wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO114WrPrivilegeOfMaster` reader - GPIO114 Write Privilege of Master"]
pub type Gpio114wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO114WrPrivilegeOfMaster` writer - GPIO114 Write Privilege of Master"]
pub type Gpio114wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO115WrPrivilegeOfMaster` reader - GPIO115 Write Privilege of Master"]
pub type Gpio115wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO115WrPrivilegeOfMaster` writer - GPIO115 Write Privilege of Master"]
pub type Gpio115wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO112 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio112wr_privilege_of_master(&self) -> Gpio112wrPrivilegeOfMasterR {
        Gpio112wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO113 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio113wr_privilege_of_master(&self) -> Gpio113wrPrivilegeOfMasterR {
        Gpio113wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO114 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio114wr_privilege_of_master(&self) -> Gpio114wrPrivilegeOfMasterR {
        Gpio114wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO115 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio115wr_privilege_of_master(&self) -> Gpio115wrPrivilegeOfMasterR {
        Gpio115wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO112 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio112wr_privilege_of_master(&mut self) -> Gpio112wrPrivilegeOfMasterW<Gpio880Spec> {
        Gpio112wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO113 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio113wr_privilege_of_master(&mut self) -> Gpio113wrPrivilegeOfMasterW<Gpio880Spec> {
        Gpio113wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO114 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio114wr_privilege_of_master(&mut self) -> Gpio114wrPrivilegeOfMasterW<Gpio880Spec> {
        Gpio114wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO115 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio115wr_privilege_of_master(&mut self) -> Gpio115wrPrivilegeOfMasterW<Gpio880Spec> {
        Gpio115wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio880::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio880::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio880Spec;
impl crate::RegisterSpec for Gpio880Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio880::R`](R) reader structure"]
impl crate::Readable for Gpio880Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio880::W`](W) writer structure"]
impl crate::Writable for Gpio880Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO880 to value 0xffff_ffff"]
impl crate::Resettable for Gpio880Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
