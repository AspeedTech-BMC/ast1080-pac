#[doc = "Register `GPIO824` reader"]
pub type R = crate::R<Gpio824Spec>;
#[doc = "Register `GPIO824` writer"]
pub type W = crate::W<Gpio824Spec>;
#[doc = "Field `GPIO020WrPrivilegeOfMaster` reader - GPIO020 Write Privilege of Master"]
pub type Gpio020wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO020WrPrivilegeOfMaster` writer - GPIO020 Write Privilege of Master"]
pub type Gpio020wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO021WrPrivilegeOfMaster` reader - GPIO021 Write Privilege of Master"]
pub type Gpio021wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO021WrPrivilegeOfMaster` writer - GPIO021 Write Privilege of Master"]
pub type Gpio021wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO022WrPrivilegeOfMaster` reader - GPIO022 Write Privilege of Master"]
pub type Gpio022wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO022WrPrivilegeOfMaster` writer - GPIO022 Write Privilege of Master"]
pub type Gpio022wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO023WrPrivilegeOfMaster` reader - GPIO023 Write Privilege of Master"]
pub type Gpio023wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO023WrPrivilegeOfMaster` writer - GPIO023 Write Privilege of Master"]
pub type Gpio023wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO020 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio020wr_privilege_of_master(&self) -> Gpio020wrPrivilegeOfMasterR {
        Gpio020wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO021 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio021wr_privilege_of_master(&self) -> Gpio021wrPrivilegeOfMasterR {
        Gpio021wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO022 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio022wr_privilege_of_master(&self) -> Gpio022wrPrivilegeOfMasterR {
        Gpio022wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO023 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio023wr_privilege_of_master(&self) -> Gpio023wrPrivilegeOfMasterR {
        Gpio023wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO020 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio020wr_privilege_of_master(&mut self) -> Gpio020wrPrivilegeOfMasterW<Gpio824Spec> {
        Gpio020wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO021 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio021wr_privilege_of_master(&mut self) -> Gpio021wrPrivilegeOfMasterW<Gpio824Spec> {
        Gpio021wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO022 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio022wr_privilege_of_master(&mut self) -> Gpio022wrPrivilegeOfMasterW<Gpio824Spec> {
        Gpio022wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO023 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio023wr_privilege_of_master(&mut self) -> Gpio023wrPrivilegeOfMasterW<Gpio824Spec> {
        Gpio023wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio824::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio824::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio824Spec;
impl crate::RegisterSpec for Gpio824Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio824::R`](R) reader structure"]
impl crate::Readable for Gpio824Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio824::W`](W) writer structure"]
impl crate::Writable for Gpio824Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO824 to value 0xffff_ffff"]
impl crate::Resettable for Gpio824Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
